use crate::phonology::feature::Feature;

use super::ConsonantFeature;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
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

impl Into<ConsonantFeature> for POA {
    fn into(self) -> ConsonantFeature {
        ConsonantFeature::POA(self)
    }
}

impl Into<Feature> for POA {
    fn into(self) -> Feature {
        Feature::Consonant(self.into())
    }
}