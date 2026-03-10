use crate::prelude::*;

pub trait DebugLayoutAlgorithm: LayoutAlgorithm {
    fn dbg_iterate(&mut self) -> Vec<Vec<Vec2>>;
}

pub trait ParameterizedDebugLayoutAlgorithm: DebugLayoutAlgorithm + Parameterized {}

impl<T> ParameterizedDebugLayoutAlgorithm for T where T: DebugLayoutAlgorithm + Parameterized {}

pub trait Parameterized {
    fn get_parameters(&self) -> Vec<Parameter>;
    fn update_parameters(&mut self, parameters: &[Parameter]);
}

#[derive(Clone, PartialEq)]
pub struct Parameter {
    name: String,
    value: ParameterValue,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ParameterValue {
    Float(f32),
    Integer(i32),
    Bool(bool),
}

pub trait FromParameterValue: Sized {
    fn from_parameter(p: &ParameterValue) -> Option<Self>;
}
impl FromParameterValue for bool {
    fn from_parameter(p: &ParameterValue) -> Option<Self> {
        match p {
            ParameterValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}
impl FromParameterValue for f32 {
    fn from_parameter(p: &ParameterValue) -> Option<Self> {
        match p {
            ParameterValue::Float(v) => Some(*v),
            _ => None,
        }
    }
}
impl FromParameterValue for i32 {
    fn from_parameter(p: &ParameterValue) -> Option<Self> {
        match p {
            ParameterValue::Integer(v) => Some(*v),
            _ => None,
        }
    }
}

pub trait ToParameterValue {
    fn to_parameter(&self) -> ParameterValue;
}
impl ToParameterValue for bool {
    fn to_parameter(&self) -> ParameterValue {
        ParameterValue::Bool(*self)
    }
}
impl ToParameterValue for f32 {
    fn to_parameter(&self) -> ParameterValue {
        ParameterValue::Float(*self)
    }
}
impl ToParameterValue for i32 {
    fn to_parameter(&self) -> ParameterValue {
        ParameterValue::Integer(*self)
    }
}
