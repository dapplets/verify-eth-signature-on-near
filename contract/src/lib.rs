use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};
use near_sdk::serde::{Serialize, Deserialize};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EcrecoverInput {
    #[serde(with = "hex::serde")]
    m: [u8; 32],
    v: u8,
    #[serde(with = "hex::serde")]
    sig: [u8; 64],
    mc: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EcrecoverOutput {
    #[serde(with = "hex::serde")]
    address: [u8; 20]
}

#[near_bindgen]
impl Counter {
    pub fn eth_ecrecover(&self, data: EcrecoverInput) -> EcrecoverOutput {
        match env::ecrecover(&data.m, &data.sig, data.v, data.mc) {
            Some(pubkey) => {
                let hash = env::keccak256_array(&pubkey);
                let address: [u8; 20] = hash[12..32].try_into().expect("Incorrect length"); // take last 20 bytes
                EcrecoverOutput { address }
            },
            None => EcrecoverOutput { address: [0; 20] },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
