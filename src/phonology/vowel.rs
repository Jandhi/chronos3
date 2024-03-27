use super::{feature::{Feature, OptionalVowelFeature, VowelFeature}, has_feature::HasFeature};

#[derive(Default, PartialEq, Eq)]
pub struct Vowel {
    pub backness : Backness,
    pub height : Height,
    pub rounded : bool,
    pub nasalized : bool,
    pub length : Length,
    pub features : Vec<OptionalVowelFeature>,
}

#[derive(PartialEq, Eq, Default)]
pub enum Backness {
    Front,
    #[default]
    Central,
    Back
}

#[derive(PartialEq, Eq, Default)]
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

#[derive(PartialEq, Eq, Default)]
pub enum Length {
    Ultrashort,
    #[default]
    Short,
    Long,
    Ultralong,
}

impl HasFeature for Vowel {
    fn has(&self, feature : &Feature) -> bool {
        match feature {
            Feature::Vowel(vowel) => {
                match vowel {
                    VowelFeature::Vowel => true,
                    VowelFeature::Backness(backness) => self.backness == *backness,
                    VowelFeature::Height(height) => self.height == *height,
                    VowelFeature::Length(length) => self.length == *length,
                    VowelFeature::Optional(opt) => self.features.contains(&opt),
                }
            },
            Feature::Consonant(_) => false,
        }
    }
}