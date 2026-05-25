// struct size is 64 to fit perfectly in CPU registers
// generation parameter is used to invalidate requests towards object indexes that got reused and are pointing at new entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Entity {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_generation(&self) -> u32 {
        self.generation
    }
}
