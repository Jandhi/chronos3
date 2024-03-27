use std::marker::PhantomData;

use super::{consonant::{MOA, POA}, vowel::{Backness, Height, Length}};

pub enum Feature {
    Vowel(VowelFeature),
    Consonant(ConsonantFeature)
}

pub enum VowelFeature {
    Vowel,
    Backness(Backness),
    Height(Height),
    Length(Length),
    Optional(OptionalVowelFeature),
}

#[derive(PartialEq, Eq)]
pub enum OptionalVowelFeature {
    Rounded,    
    Nasalized,
}

pub enum ConsonantFeature {
    Consonant,
    POA(POA),
    MOA(MOA),
    Optional(OptionalConsonantFeature)
}

#[derive(PartialEq, Eq)]
pub enum OptionalConsonantFeature {
    Voiced,
    Aspirate,
    Labialized,
    Palatalized,
    Glotallized,
}