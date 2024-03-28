use super::{moa::MOA, Consonant};
use crate::phonology::feature_traits::AddFeature;

#[macro_export]
macro_rules! C {
    ($($values:expr),*) => {
        [$($values.into()),*].iter().fold(Consonant::default(), |mut acc, ft| {
            crate::phonology::feature_traits::AddFeature::add(&mut acc, *ft);
            return acc;
        })
    };
}

fn test() {
    let mut c : Consonant = C!(MOA::Plosive);
    c.add(MOA::Plosive.into());
}