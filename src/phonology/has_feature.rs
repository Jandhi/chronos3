use super::feature::Feature;

pub trait HasFeature {
    fn has(&self, feature : &Feature) -> bool;
}