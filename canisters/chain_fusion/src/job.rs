mod calculate_result;
mod submit_result;

use std::fmt;

use ethers_core::types::{Address, U256, H160};
use evm_rpc_canister_types::LogEntry;
use ic_cdk::println;
use submit_result::{submit_question};

use crate::{
    job::calculate_result::fibonacci,
    state::{mutate_state, LogSource},
};

pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // NewJob events we can safely assume that the event is a NewJob.
    let new_question_event = NewQuestionEvent::from(event);

    // this calculation would likely exceed an ethereum blocks gas limit
    // but can easily be calculated on the IC
    // let result = fibonacci(20);

    // we write the result back to the evm smart contract, creating a signature
    // on the transaction with chain key ecdsa and sending it to the evm via the
    // evm rpc canister

    submit_question(new_question_event.creator, new_question_event.token).await;

    println!("Successfully ran job, question :{:?}", &new_question_event.question);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewQuestionEvent {
    pub creator: Address,
    pub question: String,
    pub token: U256,
}

impl fmt::Debug for NewQuestionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewQuestionEvent")
            .field("creator", &self.creator)
            .field("question", &self.question)
            .field("token", &self.token)
            .finish()
    }
}

impl From<LogEntry> for NewQuestionEvent {
    
    fn from(entry: LogEntry) -> NewQuestionEvent {
        println!("from!!!!!!!");

        // we expect exactly 2 topics from the NewJob event.
        // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
        // let job_id = U256::from_str_radix(&entry.topics[1], 16).expect("the token id should be valid");

        // let creator_address: H160 = H160::from_slice(&hex::decode(&entry.topics[1][2..]).expect("Invalid hex string"));

        let _token: U256 = U256::from(20 as u64);
        // let token = U256::from_str_radix(&entry.topics[2], 16).expect("the token id should be valid");

        let creator_address: H160 = H160::from_slice(&hex::decode("0x70997970C51812dc3A010C7d01b50e0d17dc79C8").expect("Invalid hex string"));



        // NewQuestionEvent { creator: creator_address, question: "question".to_string(), token: token }
        NewQuestionEvent { creator: creator_address, question: "sdfsd question!!".to_string(), token: _token}
    }
}
