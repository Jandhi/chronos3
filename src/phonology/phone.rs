use super::{consonant::Consonant, feature::Feature, feature_traits::HasFeature, matcher::Matchable, vowel::Vowel};

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

impl Matchable for Phone {
    fn is_match(&self, phone : Phone) -> bool {
        phone == *self
    }
}