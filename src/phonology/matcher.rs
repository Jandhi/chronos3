use super::{category::Category, phone::Phone};

pub trait Matcher {
    fn is_match(&self, phone : &Phone) -> bool;
}