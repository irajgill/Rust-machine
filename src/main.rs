
mod balances;
mod system;
#[derive(Debug)]
pub struct Runtime{
    balances: balances::Pallet,
    system: system::Pallet,
}

impl Runtime{
    pub fn new() -> Self{
        Self{
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}   


fn main() {
    let mut runtime = Runtime::new();
    let alice = "Alice".to_string();    
    let bob = "Bob".to_string();
    let charlie = "Charlie".to_string();
    runtime.balance.set_balance(&alice, 100);   

    runtime.system.inc_block_number();  
    assert_eq!(runtime.system.block_number(), 1);
    runtime.system.inc_nonce(&alice);

    let res = runtime.balances.transfer(alice.clone(), bob.clone(), 30)
    .map_err(|e| println!("Error: {}", e)); 

    println!("{#?}", runtime);


}

