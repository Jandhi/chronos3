use std::default;

use super::{feature::{ConsonantFeature, Feature, OptionalConsonantFeature}, has_feature::HasFeature};

#[derive(Default, PartialEq, Eq)]
pub struct Consonant {
    poa : POA,
    moa : MOA,
    features : Vec<OptionalConsonantFeature>,
}

#[derive(PartialEq, Eq, Default)]
pub enum POA {
    Bilabial,
    Labiodental,
    Dental,
    #[default]
    Alveolar,
    Alveopalatal,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Glottal,
    Pharyngeal,
}

#[derive(PartialEq, Eq, Default)]
pub enum MOA {
    #[default]
    Plosive,
    Affricate,
    Fricative,
    Nasal,
    Trill,
    Tap,
    Approximant,
}


impl HasFeature for Consonant {
    fn has(&self, feature : &Feature) -> bool {
        match feature {
            Feature::Vowel(_) => false,
            Feature::Consonant(ft) => match ft {
                ConsonantFeature::Consonant => true,
                ConsonantFeature::POA(poa) => self.poa == *poa,
                ConsonantFeature::MOA(moa) => self.moa == *moa,
                ConsonantFeature::Optional(opt) => self.features.contains(&opt),
            },
        }
    }
}