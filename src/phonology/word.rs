use super::phone::Phone;

pub struct Word {
    phones : Vec<Phone>,
}

impl Word {
    pub fn length(&self) -> usize {
        self.phones.len()
    }
}