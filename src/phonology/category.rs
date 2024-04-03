use super::{feature::Feature, matcher::Matcher, phone::Phone};

pub struct Category {
    features : Vec<Feature>,
    non_features : Vec<Feature>,
}

impl Matcher for Category {
    fn is_match(&self, phone : &Phone) -> bool {
        self.features.iter().all(
            |ft| phone.has(ft)
        ) && self.non_features.iter().all(
            |ft| !phone.has(ft)
        )
    }
}