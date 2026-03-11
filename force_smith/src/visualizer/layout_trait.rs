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
    pub name: String,
    pub value: ParameterValue,
}

impl Parameter {
    pub fn new(name: impl Into<String>, value: ParameterValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ParameterValue {
    Float(f32),
    Integer(i32),
    Bool(bool),
}

impl ParameterValue {
    /// Add UI element for this parameter using egui
    #[cfg(feature = "visualizer")]
    pub fn add_ui_element(&mut self, ui: &mut bevy_egui::egui::Ui) {
        match self {
            ParameterValue::Float(v) => {
                ui.add(bevy_egui::egui::DragValue::new(v).speed(0.01));
            }
            ParameterValue::Integer(v) => {
                ui.add(bevy_egui::egui::DragValue::new(v));
            }
            ParameterValue::Bool(v) => {
                ui.checkbox(v, "");
            }
        }
    }
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
