// READ | CLEAR | SET | TOGGLE
struct BitWise {
    pub value: u8,
}

impl BitWise {
    pub fn new(index: u8) -> Result<Self, String> {
        match index {
            0..8 => Ok(BitWise { value: 3 }),
            _ => Err("Choose an index between 0 and 7".to_string()),
        }
    }

    pub fn clear(&mut self, index: u8) {
        let mask: u8 = !(1u8 << index);
        self.value &= mask;
    }
}

fn main() {
    let mut bol_example = BitWise::new(1).unwrap();
    println!("{:08b}", bol_example.value);
    bol_example.clear(1);
    println!("{:08b}", bol_example.value);
}
