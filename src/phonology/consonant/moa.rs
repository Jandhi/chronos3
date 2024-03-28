use crate::phonology::feature::Feature;

use super::ConsonantFeature;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
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

impl Into<ConsonantFeature> for MOA {
    fn into(self) -> ConsonantFeature {
        ConsonantFeature::MOA(self)
    }
}

impl Into<Feature> for MOA {
    fn into(self) -> Feature {
        Feature::Consonant(self.into())
    }
}