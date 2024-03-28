use super::feature::Feature;

pub trait HasFeature {
    fn has(&self, feature : &Feature) -> bool;
}

pub trait AddFeature<TFeature> {
    fn add(&mut self, feature : TFeature);
}