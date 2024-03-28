pub mod height;
pub mod length;
pub mod backness;
pub mod optional;
pub mod builder;
mod builder_test;

use self::{backness::Backness, height::Height, length::Length, optional::OptionalVowelFeature};

use super::{feature::{CustomFeature, Feature}, feature_traits::{AddFeature, HasFeature}, phone::Phone};

#[derive(Default, PartialEq, Eq)]
pub struct Vowel {
    pub backness : Backness,
    pub height : Height,
    pub length : Length,
    pub optional : Vec<OptionalVowelFeature>,
    pub custom : Vec<CustomFeature>,
}

impl Into<Phone> for Vowel {
    fn into(self) -> Phone {
        Phone::Vowel(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VowelFeature {
    Vowel,
    Backness(Backness),
    Height(Height),
    Length(Length),
    Optional(OptionalVowelFeature),
}

impl Into<Feature> for VowelFeature {
    fn into(self) -> Feature {
        Feature::Vowel(self)
    }
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
                    VowelFeature::Optional(opt) => self.optional.contains(&opt),
                }
            },
            Feature::Consonant(_) => false,
            Feature::Custom(str) => self.custom.contains(str),
        }
    }
}

impl AddFeature<VowelFeature> for Vowel {
    fn add(&mut self, feature : VowelFeature) {
        match feature {
            VowelFeature::Vowel => {},
            VowelFeature::Backness(backness) => self.backness = backness,
            VowelFeature::Height(height) => self.height = height,
            VowelFeature::Length(length) => self.length = length,
            VowelFeature::Optional(opt) => {
                if !self.optional.contains(&opt) {
                    self.optional.push(opt);
                }
            },
        }
    }
}