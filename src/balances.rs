use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet{
    pub fn new() -> Self{
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128){
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &String) -> u128{
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, caller: String, to:String, amount: u128) -> Result<(), &'static str>{
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);
        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
     
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn init_balances(){
            let mut balances = Pallet::new();

            assert_eq!(balances.get_balance(&"Alice".to_string()), 0);
            balances.set_balance(&"Alice".to_string(), 100);

            assert_eq!(balances.get_balance(&"Alice".to_string()), 100);
            assert_eq!(balances.get_balance(&"Bob".to_string()), 0);
        }

    

        #[test]
        fn transfer(){
            let alice = "Alice".to_string();
            let bob = "Bob".to_string();
            let mut balances = super::Pallet::new();
            balances.set_balance(&alice.to_string(), 100);
            balances.set_balance(&bob.clone(), 90);    

        }

       

        #[test]
        fn transfer_insufficient_balance(){
            let alice = "Alice".to_string();
            let bob = "Bob".to_string();
            let mut balances = super::Pallet::new();
            balances.set_balance(&alice.to_string(), 100);
            balances.set_balance(&bob.clone(), 90);

            assert_eq!(balances.transfer(alice.clone(), bob.clone(), 101), Err("Insufficient balance"));
            assert_eq!(balances.get_balance(&alice), 100);
            assert_eq!(balances.get_balance(&bob), 90);
        } 
    }
