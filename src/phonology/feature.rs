
use std::rc::Rc;

use super::{consonant::ConsonantFeature, vowel::VowelFeature};

#[derive(Debug, Clone)]
pub enum Feature {
    Vowel(VowelFeature),
    Consonant(ConsonantFeature),
    Custom(CustomFeature)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFeature {
    pub name : Rc<str>
}