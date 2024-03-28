use crate::phonology::feature::Feature;

use super::ConsonantFeature;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OptionalConsonantFeature {
    Voiced,
    Aspirate,
    Labialized,
    Palatalized,
    Glotallized,
}

impl Into<ConsonantFeature> for OptionalConsonantFeature {
    fn into(self) -> ConsonantFeature {
        ConsonantFeature::Optional(self)
    }
}

impl Into<Feature> for OptionalConsonantFeature {
    fn into(self) -> Feature {
        Feature::Consonant(self.into())
    }
}