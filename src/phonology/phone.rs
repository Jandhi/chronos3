use super::{consonant::Consonant, feature::Feature, has_feature::HasFeature, matcher::Matcher, vowel::Vowel};

#[derive(PartialEq, Eq)]
pub enum Phone {
    Vowel(Vowel),
    Consonant(Consonant)
}

impl Phone {
    pub fn has(&self, feature : &Feature) -> bool {
        match self {
            Phone::Vowel(vowel) => vowel.has(feature),
            Phone::Consonant(cons) => cons.has(feature),
        }
    }
}

impl Matcher for Phone {
    fn is_match(&self, phone : &Phone) -> bool {
        phone == self
    }
}