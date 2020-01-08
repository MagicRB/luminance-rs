//! Shader API.

use crate::backend::shader::{Shader, Uniform, UniformWarning, Uniformable};
use crate::backend::shader_stage::{StageError, StageType};
use crate::context::GraphicsContext;
use std::marker::PhantomData;

pub struct Stage<S>
where
  S: Shader,
{
  repr: S::StageRepr,
}

impl<S> Stage<S>
where
  S: Shader,
{
  pub fn new<C, R>(ctx: &mut C, ty: StageType, src: R) -> Result<Self, StageError>
  where
    C: GraphicsContext<Backend = S>,
    R: AsRef<str>,
  {
    unsafe {
      ctx
        .backend()
        .new_stage(ty, src.as_ref())
        .map(|repr| Stage { repr })
    }
  }
}

impl<S> Drop for Stage<S>
where
  S: Shader,
{
  fn drop(&mut self) {
    unsafe { S::destroy_stage(&mut self.repr) }
  }
}

pub struct UniformBuilder<'a, S>
where
  S: Shader,
{
  repr: S::UniformBuilderRepr,
  _a: PhantomData<&'a mut ()>,
}

impl<'a, S> UniformBuilder<'a, S>
where
  S: Shader,
{
  pub fn ask<T, N>(&mut self, name: N) -> Result<Uniform<T>, UniformWarning>
  where
    N: AsRef<str>,
    T: Uniformable<S>,
  {
    unsafe { S::ask_uniform(&mut self.repr, name.as_ref()) }
  }

  pub fn ask_unbound<T, N>(&mut self, name: N) -> Uniform<T>
  where
    N: AsRef<str>,
    T: Uniformable<S>,
  {
    unsafe { S::ask_unbound(&mut self.repr, name.as_ref()) }
  }
}
