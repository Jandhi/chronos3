use crate::phonology::{matcher::Matchable, symbol::Symbol};

use super::{context::{self, Context}, target::Target, transformation::{self, Transformation}};

pub struct SoundChange {
    target : Target,
    transformation : Transformation,
    context : Context,
}

impl SoundChange {
    pub fn new(target : Target, transformation : Transformation, context : Context) -> SoundChange {
        SoundChange { target, transformation, context }
    }
}