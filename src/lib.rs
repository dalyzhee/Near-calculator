use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Calculator {
    num1: u32,
    num2: u32
}

#[near_bindgen]
impl Calculator {
    pub fn addition(&self) -> u32{
        self.num1 + self.num2
    }

    pub fn substraction(&self) -> u32{
        self.num1 - self.num2
    }
    pub fn multiplication(&self) -> u32{
        self.num1 * self.num2
    }
    pub fn division(&self) -> u32{
        self.num1 / self.num2
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(account: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(account);
        return builder;
    }

    // TESTS HERE
    #[test]
    fn addition(){
        let user = AccountId::new_unchecked("leviso.testnet".to_string());
        let _context = get_context(user.clone());
        let one = Calculator{
            num1: 12,
            num2: 10
        };
        let add = one.addition();
        assert_eq!(add, 22);
    }
    #[test]
    fn substraction(){
        let user = AccountId::new_unchecked("leviso.testnet".to_string());
        let _context = get_context(user.clone());
        let one = Calculator{
            num1: 12,
            num2: 10
        };
        let sub = one.substraction();
        assert_eq!(sub, 2);
    }
    #[test]
    fn multiplication(){
        let user = AccountId::new_unchecked("leviso.testnet".to_string());
        let _context = get_context(user.clone());
        let one = Calculator{
            num1: 12,
            num2: 10
        };
        let mult = one.multiplication();
        assert_eq!(mult, 120);
    }
    #[test]
    fn division(){
        let user = AccountId::new_unchecked("leviso.testnet".to_string());
        let _context = get_context(user.clone());
        let one = Calculator{
            num1: 120,
            num2: 10
        };
        let div = one.division();
        assert_eq!(div, 12);
    }

}
