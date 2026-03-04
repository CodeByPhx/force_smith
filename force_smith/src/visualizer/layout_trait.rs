use crate::layout::LayoutAlgorithm;
use bevy_egui::egui::{self, Ui};
use bevy_math::Vec2;

pub trait VisualizableDebugLayout: LayoutAlgorithm + DebugLayoutAlgorithm + Parameterized {}

impl<T> VisualizableDebugLayout for T where T: LayoutAlgorithm + DebugLayoutAlgorithm + Parameterized
{}

pub trait Parameterized {
    fn get_parameters(&self) -> Vec<(String, Parameter)>;
    fn update_parameters(&mut self, parameters: &[(String, Parameter)]);
}

pub trait DebugLayoutAlgorithm {
    fn iterate_debug(&mut self) -> Vec<Vec<Vec2>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parameter {
    Float(f32),
    Integer(i32),
    Bool(bool),
}
impl Parameter {
    pub fn add_ui_element(&mut self, ui: &mut Ui) {
        match self {
            Parameter::Float(v) => {
                ui.add(egui::DragValue::new(v).speed(0.01));
            }
            Parameter::Integer(v) => {
                ui.add(egui::DragValue::new(v).speed(1));
            }
            Parameter::Bool(v) => {
                ui.checkbox(v, ());
            }
        }
    }
}
impl From<f32> for Parameter {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}
impl From<i32> for Parameter {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}
impl From<bool> for Parameter {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub trait FromParameter: Sized {
    fn from_parameter(p: &Parameter) -> Option<Self>;
}
pub trait ToParameter {
    fn to_parameter(&self) -> Parameter;
}
impl FromParameter for bool {
    fn from_parameter(p: &Parameter) -> Option<Self> {
        if let Parameter::Bool(value) = p {
            Some(*value)
        } else {
            None
        }
    }
}
impl ToParameter for bool {
    fn to_parameter(&self) -> Parameter {
        Parameter::Bool(*self)
    }
}
impl FromParameter for f32 {
    fn from_parameter(p: &Parameter) -> Option<Self> {
        if let Parameter::Float(value) = p {
            Some(*value)
        } else {
            None
        }
    }
}
impl ToParameter for f32 {
    fn to_parameter(&self) -> Parameter {
        Parameter::Float(*self)
    }
}
