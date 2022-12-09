use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};
use near_sdk::serde::{Serialize, Deserialize};
use ethabi::token::{Token};
use ethabi::{encode,Uint,Address};

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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct EcrecoverSigInput {
    v: u8,
    #[serde(with = "hex::serde")]
    sig: [u8; 64],
    mc: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LinkingAccount {
    origin_id: String,
    account_id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LinkingAccounts {
    account_a: LinkingAccount,
    account_b: LinkingAccount,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HashOutput {
    #[serde(with = "hex::serde")]
    hash: [u8; 32]
}

fn _domain_separator() -> [u8; 32] {
    let chain_id = 5;
    let verifying_contract = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let eip712_domain = "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    let name = "Connected Accounts";
    let version = "1";
    let eip712_domain_typehash = env::keccak256_array(&eip712_domain.as_bytes());
    let domain_separator = env::keccak256_array(encode(&[
        Token::FixedBytes(eip712_domain_typehash.to_vec()),
        Token::FixedBytes(env::keccak256_array(&name.as_bytes()).to_vec()),
        Token::FixedBytes(env::keccak256_array(&version.as_bytes()).to_vec()),
        Token::Uint(Uint::from(chain_id)),
        Token::Address(Address::from(verifying_contract)),
    ]).as_slice());

    domain_separator
}

fn _hash_linking_accounts_no_domain(linking_accounts: LinkingAccounts) -> [u8; 32] {
    let linking_accounts_type = "LinkingAccounts(LinkingAccount account_a,LinkingAccount account_b)LinkingAccount(string origin_id,string account_id)";
    let linking_accounts_typehash = env::keccak256_array(&linking_accounts_type.as_bytes());

    env::keccak256_array(
        encode(&[
            Token::FixedBytes(linking_accounts_typehash.to_vec()),
            Token::FixedBytes(_hash_linking_account(linking_accounts.account_a).to_vec()),
            Token::FixedBytes(_hash_linking_account(linking_accounts.account_b).to_vec()),
        ]).as_slice()
    )
}

fn _hash_linking_accounts(linking_accounts: LinkingAccounts) -> [u8; 32] {
    let domain_separator = _domain_separator();
    let hash_linking_accounts_no_domain = _hash_linking_accounts_no_domain(linking_accounts);

    let prefix = [0x19, 0x01].to_vec(); // "\\x19\\x01"
    let postfix = encode(&[
        Token::FixedBytes(domain_separator.to_vec()),
        Token::FixedBytes(hash_linking_accounts_no_domain.to_vec())
    ]);
    let concated = [prefix, postfix].concat();

    env::keccak256_array(concated.as_slice())
}

fn _hash_linking_account(linking_account: LinkingAccount) -> [u8; 32] {
    let linking_account_type = "LinkingAccount(string origin_id,string account_id)";
    let linking_account_typehash = env::keccak256_array(&linking_account_type.as_bytes());

    env::keccak256_array(encode(&[
        Token::FixedBytes(linking_account_typehash.to_vec()),
        Token::FixedBytes(env::keccak256_array(linking_account.origin_id.as_bytes()).to_vec()), // strings must be hashed
        Token::FixedBytes(env::keccak256_array(linking_account.account_id.as_bytes()).to_vec()), // strings must be hashed
    ]).as_slice())
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

    pub fn eth_verify_eip712(&self, linking_accounts: LinkingAccounts, signature: EcrecoverSigInput) -> EcrecoverOutput {
        let hash = _hash_linking_accounts(linking_accounts);
        match env::ecrecover(&hash, &signature.sig, signature.v, signature.mc) {
            Some(pubkey) => {
                let hash = env::keccak256_array(&pubkey);
                let address: [u8; 20] = hash[12..32].try_into().expect("Incorrect length"); // take last 20 bytes
                EcrecoverOutput { address }
            },
            None => EcrecoverOutput { address: [0; 20] },
        }
    }

    // FOR UNIT TEST ONLY

    pub fn domain_separator(&self) -> HashOutput {
        let hash = _domain_separator();
        HashOutput{ hash: hash }
    } 

    pub fn hash_linking_accounts(&self, linking_accounts: LinkingAccounts) -> HashOutput {
        let hash = _hash_linking_accounts(linking_accounts);
        HashOutput{ hash: hash }
    }

    pub fn hash_linking_account(&self, linking_account: LinkingAccount) -> HashOutput {
        let hash = _hash_linking_account(linking_account);
        HashOutput{ hash: hash }
    }

    pub fn hash_linking_accounts_no_domain(&self, linking_accounts: LinkingAccounts ) -> HashOutput {
        let hash = _hash_linking_accounts_no_domain(linking_accounts);
        HashOutput { hash: hash }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
