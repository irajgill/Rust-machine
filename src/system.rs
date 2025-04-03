use std::collections::BTreeMap; 

#[derive(Debug)]
pub struct Pallet{
    blocknumber: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet{
    pub fn new() -> Self{
        Self{
            blocknumber: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32{
        self.block_number;
    }

    pub fn inc_block_number(&mut self){
        self.block_number = self.block_number.checked_add(1).unwrap();
    
    }

    pub fn get_nonce(&self, who: &String) -> u32{
        self.nonce.get(who).unwrap_or(&0)
    }   

    pub fn inc_nonce(&self, who: &String) -> u32{
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }



}

#[cfg(test)]
mod test{

fn init_system(){
    let system = Pallet::new();
    assert_eq!(system.block_number(), 0);
}

[#test]
fn inc_block_number(){
    let mut system = Pallet::new();
    system.inc_block_number();
    assert_eq!(system.block_number(), 1);
}

[#test]
fn inc_nonce(){
    let alice = String::from("Alice");
    let mut system = super::Pallet::new();
    system.inc_nonce(&alice.clone() );
    assert_eq!(system.get_nonce(&alice).unwrap(), 1);
  ;
    
}
}