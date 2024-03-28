use super::{feature::Feature, matcher::Matchable};

pub struct Category {
    features : Vec<Feature>,
    non_features : Vec<Feature>,
}

impl Matchable for Category {
    fn is_match(&self, phone : super::phone::Phone) -> bool {
        self.features.iter().all(
            |ft| phone.has(ft)
        ) && self.non_features.iter().all(
            |ft| !phone.has(ft)
        )
    }
}