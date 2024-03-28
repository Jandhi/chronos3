use super::{category::Category, matcher::Matchable, phone::Phone};

pub enum Symbol {
    Category(Category),
    Phone(Phone),
}

impl Matchable for Symbol {
    fn is_match(&self, phone : Phone) -> bool {
        match self {
            Symbol::Category(cat) => cat.is_match(phone),
            Symbol::Phone(ph) => ph.is_match(phone),
        }
    }
}