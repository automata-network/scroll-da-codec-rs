use rpc::{ProveBatchRequest, ProveBatchResponse, ProveBundleRequest, ProveBundleResponse};
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{block_tracer::BlockTracer, prover::Prover, types::{CommitBatchEvent, FinalizeBatchEvent}};
use std::sync::Arc;
use crate::state_manager::StateManager;


pub struct TaskManager {
}

impl TaskManager {
    pub fn new() -> Self {
        todo!()
    }

    async fn prove_batch(prover: Arc<Prover>, request: ProveBatchRequest) -> ProveBatchResponse {
        loop {
            match prover.prove_batch(request.clone()).await {
                Ok(resp) => {
                    break resp;
                },
                Err(err) => {
                    // todo add log
                    tokio::time::sleep(core::time::Duration::from_secs(5));
                }
            }
        }
    }

    async fn prove_bundle_and_submit(prover: Arc<Prover>, request: ProveBundleRequest) -> ProveBundleResponse {
        let response = loop {
            match prover.prove_bundle(request.clone()).await {
                Ok(resp) => {
                    break resp;
                },
                Err(err) => {
                    // todo add log
                    tokio::time::sleep(core::time::Duration::from_secs(5));
                }
            }
        };
        loop {
            prover.submit_bundle_proof(response).await
        }
    }

    async fn handle_batch_event(
        proof_state: Arc<StateManager>,
        prover: Arc<Prover>,
        block_tracer: Arc<BlockTracer>,
        mut rx: Receiver<CommitBatchEvent>,
        prove_batch_tx: Sender<ProveBatchResponse>
    ) -> () {
        while let Some(event) = rx.recv().await {
            if let Ok(request) = proof_state.on_batch_commit_event_received(event, block_tracer.clone()).await {
                let response = TaskManager::prove_batch(prover.clone(), request).await;
                prove_batch_tx.send(response).await;
            } else {
                // todo, retry or handle error
            }
        }
    }

    async fn handle_bundle_event(
        proof_state: Arc<StateManager>,
        prover: Arc<Prover>,
        mut rx: Receiver<FinalizeBatchEvent>,
        mut prove_batch_rx: Receiver<ProveBatchResponse>) -> () {
        
        loop {
            let requests = tokio::select! {
                finalize_batch_option = rx.recv() => {
                    match finalize_batch_option {
                        Some(finalize_batch_event) => {
                            if let Ok(reqs) = proof_state.on_batch_finalize_event_received(finalize_batch_event).await {
                                reqs
                            } else {
                                vec![]
                            }
                        },
                        None => break
                    }
                }
                proved_batch_option = prove_batch_rx.recv() => {
                    match proved_batch_option {
                        Some(prove_batch_response) => {
                            if let Ok(reqs) = proof_state.on_batch_proved(prove_batch_response).await {
                                reqs
                            } else {
                                vec![]
                            }
                        },
                        None => break
                    }
                }
            };
            for request in requests {
                TaskManager::prove_bundle_and_submit(prover.clone(), request).await;
            }
        }
    }

    pub async fn start(&self,
        commit_batch_event_rx: Receiver<CommitBatchEvent>,
        finalize_batch_event_rx: Receiver<FinalizeBatchEvent>,
    ) {
        let proof_state_manager = Arc::new(StateManager::new());
        let prover = Arc::new(Prover::new());
        let block_tracer = Arc::new(BlockTracer::new());

        let (prove_batch_resp_tx, prove_batch_resp_rx) = mpsc::channel::<ProveBatchResponse>(32);

        let proof_state_manager_copy = proof_state_manager.clone();
        let prover_copy = prover.clone();
        tokio::spawn(async move {
            TaskManager::handle_batch_event(
                proof_state_manager_copy, 
                prover_copy,
                block_tracer,
                commit_batch_event_rx,
                prove_batch_resp_tx);
        });

        tokio::spawn(async move {
            TaskManager::handle_bundle_event(
                proof_state_manager, 
                prover,
                finalize_batch_event_rx,
                prove_batch_resp_rx);
        });

        ()
    }
}