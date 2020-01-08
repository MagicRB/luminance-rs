//! Shader backend.

use std::fmt;
use std::marker::PhantomData;

use crate::backend::shader_stage::{StageError, StageType, TessellationStages};

/// Errors that a `Program` can generate.
#[derive(Debug)]
pub enum ProgramError {
  /// A shader stage failed to compile or validate its state.
  StageError(StageError),
  /// Program link failed. You can inspect the reason by looking at the contained `String`.
  LinkFailed(String),
  /// Some uniform configuration is ill-formed. It can be a problem of inactive uniform, mismatch
  /// type, etc. Check the `UniformWarning` type for more information.
  UniformWarning(UniformWarning),
  /// Some vertex attribute is ill-formed.
  VertexAttribWarning(VertexAttribWarning),
}

impl fmt::Display for ProgramError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      ProgramError::StageError(ref e) => write!(f, "shader program has stage error: {}", e),

      ProgramError::LinkFailed(ref s) => write!(f, "shader program failed to link: {}", s),

      ProgramError::UniformWarning(ref e) => {
        write!(f, "shader program contains uniform warning(s): {}", e)
      }
      ProgramError::VertexAttribWarning(ref e) => write!(
        f,
        "shader program contains vertex attribute warning(s): {}",
        e
      ),
    }
  }
}

/// Warnings related to uniform issues.
#[derive(Debug)]
pub enum UniformWarning {
  /// Inactive uniform (not in use / no participation to the final output in shaders).
  Inactive(String),
  /// Type mismatch between the static requested type (i.e. the `T` in [`Uniform<T>`] for instance)
  /// and the type that got reflected from the backend in the shaders.
  ///
  /// The first `String` is the name of the uniform; the second one gives the type mismatch.
  TypeMismatch(String, StageType),
}

impl fmt::Display for UniformWarning {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      UniformWarning::Inactive(ref s) => write!(f, "inactive {} uniform", s),

      UniformWarning::TypeMismatch(ref n, ref t) => {
        write!(f, "type mismatch for uniform {}: {}", n, t)
      }
    }
  }
}

/// Warnings related to vertex attributes issues.
#[derive(Debug)]
pub enum VertexAttribWarning {
  /// Inactive vertex attribute (not read).
  Inactive(String),
}

impl fmt::Display for VertexAttribWarning {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      VertexAttribWarning::Inactive(ref s) => write!(f, "inactive {} vertex attribute", s),
    }
  }
}

pub struct Uniform<T>
where
  T: ?Sized,
{
  index: i32,
  _t: PhantomData<*const T>,
}

/// Type of a uniform.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UniformType {
  // scalars
  /// 32-bit signed integer.
  Int,
  /// 32-bit unsigned integer.
  UInt,
  /// 32-bit floating-point number.
  Float,
  /// Boolean.
  Bool,

  // vectors
  /// 2D signed integral vector.
  IVec2,
  /// 3D signed integral vector.
  IVec3,
  /// 4D signed integral vector.
  IVec4,
  /// 2D unsigned integral vector.
  UIVec2,
  /// 3D unsigned integral vector.
  UIVec3,
  /// 4D unsigned integral vector.
  UIVec4,
  /// 2D floating-point vector.
  Vec2,
  /// 3D floating-point vector.
  Vec3,
  /// 4D floating-point vector.
  Vec4,
  /// 2D boolean vector.
  BVec2,
  /// 3D boolean vector.
  BVec3,
  /// 4D boolean vector.
  BVec4,

  // matrices
  /// 2×2 floating-point matrix.
  M22,
  /// 3×3 floating-point matrix.
  M33,
  /// 4×4 floating-point matrix.
  M44,

  // textures
  /// Signed integral 1D texture sampler.
  ISampler1D,
  /// Signed integral 2D texture sampler.
  ISampler2D,
  /// Signed integral 3D texture sampler.
  ISampler3D,
  /// Unsigned integral 1D texture sampler.
  UISampler1D,
  /// Unsigned integral 2D texture sampler.
  UISampler2D,
  /// Unsigned integral 3D texture sampler.
  UISampler3D,
  /// Floating-point 1D texture sampler.
  Sampler1D,
  /// Floating-point 2D texture sampler.
  Sampler2D,
  /// Floating-point 3D texture sampler.
  Sampler3D,
  /// Signed cubemap sampler.
  ICubemap,
  /// Unsigned cubemap sampler.
  UICubemap,
  /// Floating-point cubemap sampler.
  Cubemap,

  // buffer
  /// Buffer binding; used for UBOs.
  BufferBinding,
}

impl fmt::Display for UniformType {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      UniformType::Int => f.write_str("int"),
      UniformType::UInt => f.write_str("uint"),
      UniformType::Float => f.write_str("float"),
      UniformType::Bool => f.write_str("bool"),
      UniformType::IVec2 => f.write_str("ivec2"),
      UniformType::IVec3 => f.write_str("ivec3"),
      UniformType::IVec4 => f.write_str("ivec4"),
      UniformType::UIVec2 => f.write_str("uvec2"),
      UniformType::UIVec3 => f.write_str("uvec3"),
      UniformType::UIVec4 => f.write_str("uvec4"),
      UniformType::Vec2 => f.write_str("vec2"),
      UniformType::Vec3 => f.write_str("vec3"),
      UniformType::Vec4 => f.write_str("vec4"),
      UniformType::BVec2 => f.write_str("bvec2"),
      UniformType::BVec3 => f.write_str("bvec3"),
      UniformType::BVec4 => f.write_str("bvec4"),
      UniformType::M22 => f.write_str("mat2"),
      UniformType::M33 => f.write_str("mat3"),
      UniformType::M44 => f.write_str("mat4"),
      UniformType::ISampler1D => f.write_str("isampler1D"),
      UniformType::ISampler2D => f.write_str("isampler2D"),
      UniformType::ISampler3D => f.write_str("isampler3D"),
      UniformType::UISampler1D => f.write_str("uSampler1D"),
      UniformType::UISampler2D => f.write_str("uSampler2D"),
      UniformType::UISampler3D => f.write_str("uSampler3D"),
      UniformType::Sampler1D => f.write_str("sampler1D"),
      UniformType::Sampler2D => f.write_str("sampler2D"),
      UniformType::Sampler3D => f.write_str("sampler3D"),
      UniformType::ICubemap => f.write_str("isamplerCube"),
      UniformType::UICubemap => f.write_str("usamplerCube"),
      UniformType::Cubemap => f.write_str("samplerCube"),
      UniformType::BufferBinding => f.write_str("buffer binding"),
    }
  }
}

pub unsafe trait Uniformable<S>
where
  S: ?Sized + Shader,
{
  unsafe fn update(self, program: &mut S::ProgramRepr, uniform: &mut Uniform<Self>);

  unsafe fn ty() -> UniformType;
}

pub unsafe trait Shader {
  type StageRepr;

  type ProgramRepr;

  type UniformBuilderRepr;

  unsafe fn new_stage(&mut self, ty: StageType, src: &str) -> Result<Self::StageRepr, StageError>;

  unsafe fn destroy_stage(stage: &mut Self::StageRepr);

  unsafe fn from_stages(
    &mut self,
    vertex: &Self::StageRepr,
    tess: Option<TessellationStages<Self::StageRepr>>,
    geometry: Option<&Self::StageRepr>,
    fragment: &Self::StageRepr,
  ) -> Result<Self::ProgramRepr, ProgramError>;

  unsafe fn new_uniform_builder(
    program: &mut Self::ProgramRepr,
  ) -> Result<Self::UniformBuilderRepr, ProgramError>;

  unsafe fn ask_uniform<T>(
    uniform_builder: &mut Self::UniformBuilderRepr,
    name: &str,
  ) -> Result<Uniform<T>, UniformWarning>
  where
    T: Uniformable<Self>;

  unsafe fn ask_unbound<T>(
    uniform_builder: &mut Self::UniformBuilderRepr,
    name: &str,
  ) -> Uniform<T>
  where
    T: Uniformable<Self>;
}
