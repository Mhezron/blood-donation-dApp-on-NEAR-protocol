#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::num;
use std::task::Context;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::predecessor_account_id;
use near_sdk::{env, near_bindgen, log, Timestamp};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Donations {
    donations: HashMap<u64, Donation>,
    ids: u64,
}

impl Default for Donations {
    fn default() -> Self {
        Self {
            donations: HashMap::new(),
            ids: 0,
            
        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Donation{
    title: String,
    description: String,
    pints:u16,
    blood_type: String,
    time: Timestamp,
    location: String,
    donators: HashMap<String, String>,
    patient: String,
    votes: u64,
}


#[near_bindgen]
impl Donations {
    pub fn new_donation(&mut self, title: String, description: String, pints: u16, blood_type: String, location: String) { 
        let patient = env::predecessor_account_id().to_string();
        let id = self.ids + 1;
        let new_donation = Donation {
            title, 
            description, 
            pints, 
            blood_type, 
            time: env::block_timestamp(), 
            location,
            donators: HashMap::new(),
            patient,
            votes: 0,
        };
        self.donations.insert(id, new_donation);
    }

    /// view list of availanble donations
    pub fn view_donations(&self) {
        for donation in &self.donations {
            log!("id: {}", donation.0);
            log!("{} ", donation.1.title);
            log!("{} ", donation.1.description);
            log!("{} ", donation.1.pints);
            log!("{} ", donation.1.blood_type);
            log!("{} ", donation.1.time);
            log!("{} ", donation.1.location);
        }
    }

    // Can only be viewed by patient who initiated the blood drive
    pub fn view_donators(&self, id: u64) {
        let viewer_id = env::predecessor_account_id().to_string();
        if viewer_id == self.donations[&id].patient {
            for donator in &self.donations[&id].donators {
                log!("id: {}, Number: {}", donator.0, donator.1);
            }
        }
    }

    pub fn donate(&mut self, number: String, id: u64) {
        let donator_id = env::predecessor_account_id().to_string();
        if self.donations.contains_key(&id) {
            if let Some(donations_struct) = self.donations.get_mut(&id) {
                donations_struct.donators.insert(donator_id, number);
            }
        }
    }

    pub fn vote(&mut self, id: u64) {
        if self.donations.contains_key(&id) {
            if let Some(donations_struct) = self.donations.get_mut(&id) {
                donations_struct.votes += 1;
            }
        }
    }

}

#[cfg(test)]
mod tests {}
    //use super::*;
    use near_sdk::{MockedBlockchain, AccountId};
    use near_sdk::{testing_env, VMContext};

   // part of writing unit tests is setting up a mock context
    // mock the context for testing, notice "signer_account_id" that was accessed above from env::

    fn patient() -> AccountId {
        AccountId::try_from("patient.testnet".to_string().clone()).unwrap()   
    }
    
    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: AccountId::try_from("mhezron.testnet".to_string().clone()).unwrap(),
            signer_account_id: patient(),
            signer_account_pk: vec![0u8; 33].try_into().unwrap(),
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit:0,
            view_config: None,
            prepaid_gas: near_sdk::Gas(10u64.pow(18)),
            random_seed: [0u8; 32],
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }


    //running individual test functions

#[test]

fn test_new_donation() {
    let context = get_context(patient());
    testing_env!(context);
    let mut contract: Donations = Donations::default();
    contract.new_donation("Joe's-blood-drive".to_string(),
     "help us save Joe's life".to_string(), 
     4, 
     "A+".to_string(), 
     "Mombasa".to_string());
     assert_eq!(true, contract.donations.contains_key(&1));

}

pub fn st(text: &str) -> String{
    text.to_string()
}

fn test_view_donation(){
    let context = get_context(patient());
    testing_env!(context);
    let contract: Donations = Donations::default();
    contract.view_donations( )
}   