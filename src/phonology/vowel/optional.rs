use crate::phonology::feature::Feature;

use super::VowelFeature;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum OptionalVowelFeature {
    Rounded,    
    Nasalized,
    Stressed,
}

impl Into<VowelFeature> for OptionalVowelFeature {
    fn into(self) -> VowelFeature {
        VowelFeature::Optional(self)
    }
}

impl Into<Feature> for OptionalVowelFeature {
    fn into(self) -> Feature {
        Feature::Vowel(self.into())
    }
}