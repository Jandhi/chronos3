use std::{io::Read, marker::PhantomData};

use super::{context::Context, sound_change::SoundChange, target::Target, transformation::Transformation};

pub struct SoundChangeBuilder<T> {
    t : PhantomData<T>,
    target : Target,
    transformation : Transformation,
    context : Context
}

pub struct NeedsTransformationState;
pub struct NeedsContextState;
pub struct FinalState;

pub trait Ready {}
impl Ready for NeedsContextState {}
impl Ready for FinalState {}

pub fn target<T>(target : T) -> SoundChangeBuilder<NeedsTransformationState> 
where T : Into<Target>
{
    SoundChangeBuilder { t : Default::default(), target : target.into(), transformation : Default::default(), context : Default::default() }
}

impl SoundChangeBuilder<NeedsTransformationState> {
    pub fn becomes<T>(self, transformation : T) -> SoundChangeBuilder<NeedsContextState>
    where T : Into<Transformation>
    {
        SoundChangeBuilder { t : Default::default(), target : self.target, transformation : transformation.into(), context : self.context  }
    }
}

impl SoundChangeBuilder<NeedsContextState> {
    pub fn when<T>(self, ctx : T) -> SoundChangeBuilder<FinalState>
    where T : Into<Context>
    {
        SoundChangeBuilder { t : Default::default(), target : self.target, transformation : self.transformation, context : ctx.into()  }
    }
}

impl<TBuilderState> Into<SoundChange> for SoundChangeBuilder<TBuilderState>
where TBuilderState : Ready
{
    fn into(self) -> SoundChange {
        SoundChange::new(
            self.target,
            self.transformation,
            self.context,
        )
    }
}