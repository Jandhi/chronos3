use crate::phonology::feature::Feature;

use super::VowelFeature;

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum Length {
    Ultrashort,
    #[default]
    Short,
    Long,
    Ultralong,
}

impl Into<VowelFeature> for Length {
    fn into(self) -> VowelFeature {
        VowelFeature::Length(self)
    }
}

impl Into<Feature> for Length {
    fn into(self) -> Feature {
        Feature::Vowel(self.into())
    }
}