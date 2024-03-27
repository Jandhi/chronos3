use super::{category::Category, matcher::Matcher, phone::Phone};

pub enum Target {
    Category(Category),
    Phone(Phone),
}

impl Matcher for Target {
    fn is_match(&self, phone : Phone) -> bool {
        match self {
            Target::Category(cat) => cat.is_match(phone),
            Target::Phone(ph) => ph.is_match(phone),
        }
    }
}
