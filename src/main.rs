use bitwise_operators::Index;

// READ | CLEAR | SET | TOGGLE
#[derive(Clone, Copy)]
struct BitWise{
    pub value: u8,
}

impl BitWise {
    pub fn new(index: Index) -> Result<Self, String> {
        Ok(BitWise{value:1})
    }

    pub fn clear(&mut self, index: Index) {
        let mask: u8 = !(1u8 << index.get());
        self.value &= mask;
    }
  
    pub fn read(self,index: Index) -> bool {
        self.value >> index.get() & 1 == 1
    }

    pub fn set(&mut self,index: Index, new_value: bool ) {
        if new_value == true{
            self.value = self.value | (1 << index.get());
        }else{
            self.value = self.value & !(1 << index.get());
        }
    }
}

fn main() {
    // let mut bol_example = BitWise::new(1.into()).unwrap();
    // println!("{:08b}", bol_example.value);
    // bol_example.clear(1.into());
    // println!("{:08b}", bol_example.value);
    // let mut bit = BitWise {value:1};
    // for i in 0..=7{
    //     println!(" index {i} = {}", bit.read(i.into()).unwrap());
    // }

    // bit.set(1.into(), false);
    //     println!();

    // for i in 0..=7{
    //     println!(" index {i} = {}", bit.read(i.into()).unwrap());
    // }
}
