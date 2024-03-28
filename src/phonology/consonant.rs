pub mod poa;
pub mod moa;
pub mod optional;
pub mod builder;

use self::{moa::MOA, optional::OptionalConsonantFeature, poa::POA};

use super::{feature::{CustomFeature, Feature}, feature_traits::{AddFeature, HasFeature}, phone::Phone};

#[derive(Default, PartialEq, Eq)]
pub struct Consonant {
    poa : POA,
    moa : MOA,
    optional : Vec<OptionalConsonantFeature>,
    custom : Vec<CustomFeature>,
}

impl Into<Phone> for Consonant {
    fn into(self) -> Phone {
        Phone::Consonant(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConsonantFeature {
    Consonant,
    POA(POA),
    MOA(MOA),
    Optional(OptionalConsonantFeature)
}

impl Into<Feature> for ConsonantFeature {
    fn into(self) -> Feature {
        Feature::Consonant(self)
    }
}

impl HasFeature for Consonant {
    fn has(&self, feature : &Feature) -> bool {
        match feature {
            Feature::Vowel(_) => false,
            Feature::Consonant(ft) => match ft {
                ConsonantFeature::Consonant => true,
                ConsonantFeature::POA(poa) => self.poa == *poa,
                ConsonantFeature::MOA(moa) => self.moa == *moa,
                ConsonantFeature::Optional(opt) => self.optional.contains(&opt),
            },
            Feature::Custom(str) => self.custom.contains(str),
        }
    }
}

impl AddFeature<ConsonantFeature> for Consonant {
    fn add(&mut self, feature : ConsonantFeature) {
        match feature {
            ConsonantFeature::Consonant => {},
            ConsonantFeature::POA(poa) => self.poa = poa,
            ConsonantFeature::MOA(moa) => self.moa = moa,
            ConsonantFeature::Optional(opt) => {
                if !self.optional.contains(&opt) {
                    self.optional.push(opt);
                }
            },
        }
    }
}