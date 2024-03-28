use super::{backness::Backness, Vowel};
use crate::phonology::feature_traits::AddFeature;

#[macro_export]
macro_rules! V {
    ($($values:expr),*) => {
        [$($values.into()),*].iter().fold(Vowel::default(), |mut acc, ft| {
            crate::phonology::feature_traits::AddFeature::add(&mut acc, *ft);
            return acc;
        })
    };
}

fn test() {
    let mut v : Vowel = V!(Backness::Front);
    v.add(Backness::Front.into());
}