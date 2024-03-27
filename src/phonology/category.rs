use super::{feature::Feature, matcher::Matcher};

pub struct Category {
    features : Vec<Feature>,
    non_features : Vec<Feature>,
}

impl Matcher for Category {
    fn is_match(&self, phone : super::phone::Phone) -> bool {
        self.features.iter().all(
            |ft| phone.has(ft)
        ) && self.non_features.iter().all(
            |ft| !phone.has(ft)
        )
    }
}