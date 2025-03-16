struct Contract has 1 vari messages.
messages is a Vector of PostedMessages

Theres an Default impl for contract,
in that in it might seem that the default value of messages is Vector::new(b"m")

Contract then has a payable macro on add_message?

checks if its premium by checking if required amount of deposit is attached
caches sender as predecssor_account_id() // seems to be msg.sender equivalent

message = PostedMessage {
    premium,sender,text
}
self.messages.push(message);
// get messages travesses the messages array

total messages queries len of messages

```rust
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]//Serialize is a trait from the serde module (included in near-sdk), a popular Rust library for serialization.It allows Rust types to be converted into formats like JSON, which is how NEAR contracts typically return data to callers.
#[borsh(crate = "near_sdk::borsh")]//Guarantees the use of the correct Borsh crate.
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)] // Implements serialization/deserialization for the struct’s state using Borsh, enabling persistence
#[borsh(crate = "near_sdk::borsh")] // Ensures the NEAR SDK’s version of Borsh is used, avoiding compatibility issues.
pub struct Contract {
    messages: Vector<PostedMessage>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"), // TODO: What does this mean
        }
    }
}

#[near_bindgen] // TODO: Macro. Turns the Contract struct into a NEAR smart contract, exposing its methods and managing blockchain interactions.
impl Contract {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        self.messages.push(message);
    }

    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages();
        assert!(*total == 3);

        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        assert_eq!(last_message.premium, false);
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}
```

Summary:
Contract that stores messages in a state variable of type `Vector<PostedMessages>`
Allows to add messages, query to get all messages, and get total num of messages.
Premium messages are indicated via reaching threshold amount of `env::attached_deposit()`
Index of messages is determed by first come first serve
TestSuite to ensure that it works