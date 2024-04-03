
use super::{category::Category, matcher::Matcher, phone::Phone};

pub struct Target {
    units : Vec<TargetUnit>,
}

impl Target {
    pub fn is_match(&self, phones : Vec<Phone>) -> bool {
        if phones.len() != self.units.len() {
            return false;
        }

        for (phone, unit) in phones.iter().zip(self.units.iter()) {
            if !unit.is_match(phone) {
                return false;
            }
        }

        return true;
    }
}

pub enum TargetUnit {
    Category(Category),
    Phone(Phone),
}

impl Matcher for TargetUnit {
    fn is_match(&self, phone : &Phone) -> bool {
        match self {
            TargetUnit::Category(cat) => cat.is_match(phone),
            TargetUnit::Phone(ph) => ph.is_match(phone),
        }
    }
}
