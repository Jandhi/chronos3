use crate::phonology::feature::Feature;

use super::VowelFeature;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum Backness {
    Front,
    #[default]
    Central,
    Back
}

impl Into<VowelFeature> for Backness {
    fn into(self) -> VowelFeature {
        VowelFeature::Backness(self)
    }
}

impl Into<Feature> for Backness {
    fn into(self) -> Feature {
        Feature::Vowel(self.into())
    }
}