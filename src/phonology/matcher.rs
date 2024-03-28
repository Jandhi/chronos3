use super::{category::Category, phone::Phone};

pub trait Matchable {
    fn is_match(&self, phone : Phone) -> bool;
}