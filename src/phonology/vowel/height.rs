use crate::phonology::feature::Feature;

use super::VowelFeature;

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum Height {
    High,
    NearHigh,
    MidHigh,
    Mid,
    MidLow,
    NearLow,
    #[default]
    Low
}

impl Into<VowelFeature> for Height {
    fn into(self) -> VowelFeature {
        VowelFeature::Height(self)
    }
}

impl Into<Feature> for Height {
    fn into(self) -> Feature {
        Feature::Vowel(self.into())
    }
}