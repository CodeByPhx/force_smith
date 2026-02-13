use bevy_math::Vec2;

pub struct SafeVec2<'a>(pub &'a Vec2);
impl<'a> From<&'a Vec2> for SafeVec2<'a> {
    fn from(value: &'a Vec2) -> Self {
        Self(value)
    }
}
pub struct SafeMutVec2<'a>(pub &'a mut Vec2);
impl<'a> From<&'a mut Vec2> for SafeMutVec2<'a> {
    fn from(value: &'a mut Vec2) -> Self {
        Self(value)
    }
}
//
//
// #[cfg_attr(feature = "visualizer", derive(serde::Serialize, serde::Deserialize))]
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Vec2 {
//     pub x: f32,
//     pub y: f32,
// }
//
// impl Vec2 {
//     pub const ZERO: Self = Vec2 { x: 0.0, y: 0.0 };
//     pub const ONE: Self = Vec2 { x: 1.0, y: 1.0 };
//
//     pub fn normalize(&self) -> Option<Self> {
//         let normalized = self / self.length();
//         if normalized.is_any_nan() {
//             None
//         } else {
//             Some(normalized)
//         }
//     }
//
//     pub fn is_any_nan(&self) -> bool {
//         self.x.is_nan() || self.y.is_nan()
//     }
//     pub fn length(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
//     pub fn new(x: f32, y: f32) -> Self {
//         Self::from_xy(x, y)
//     }
//     pub fn from_xy(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }
//     pub fn splat(v: f32) -> Self {
//         Self::from_xy(v, v)
//     }
//     pub fn distance(&self, other: &Self) -> f32 {
//         self.direction(other).length()
//     }
//     pub fn direction(&self, other: &Self) -> Self {
//         other - self
//     }
//
//     pub fn clamp_length<R>(&self, range: R) -> Option<Self>
//     where
//         R: RangeBounds<f32>,
//     {
//         let len = self.length();
//
//         let min = match range.start_bound() {
//             std::ops::Bound::Included(v) => *v,
//             std::ops::Bound::Excluded(v) => f32::from_bits(v.to_bits() + 1),
//             std::ops::Bound::Unbounded => f32::NEG_INFINITY,
//         };
//
//         let max = match range.start_bound() {
//             std::ops::Bound::Included(v) => *v,
//             std::ops::Bound::Excluded(v) => f32::from_bits(v.to_bits() - 1),
//             std::ops::Bound::Unbounded => f32::INFINITY,
//         };
//
//         if len < min {
//             self.normalize().map(|v| v * min)
//         } else if len > max {
//             self.normalize().map(|v| v * max)
//         } else {
//             Some(*self)
//         }
//     }
// }
//
// impl Position for Vec2 {
//     fn position(&self) -> Vec2 {
//         *self
//     }
// }
//
// impl std::ops::Add for Vec2 {
//     type Output = Vec2;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Vec2 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
// impl std::ops::Add for &Vec2 {
//     type Output = Vec2;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Vec2 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
// impl std::ops::AddAssign for Vec2 {
//     fn add_assign(&mut self, rhs: Self) {
//         self.x += rhs.x;
//         self.y += rhs.y;
//     }
// }
//
// impl std::ops::Sub for Vec2 {
//     type Output = Vec2;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Vec2 {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }
// impl std::ops::Sub for &Vec2 {
//     type Output = Vec2;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Vec2 {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }
// impl std::ops::SubAssign for Vec2 {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.x -= rhs.x;
//         self.y -= rhs.y;
//     }
// }
//
// impl std::ops::Mul<f32> for Vec2 {
//     type Output = Vec2;
//
//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec2 {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }
// impl std::ops::Mul<f32> for &Vec2 {
//     type Output = Vec2;
//
//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec2 {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }
// impl std::ops::MulAssign<f32> for Vec2 {
//     fn mul_assign(&mut self, rhs: f32) {
//         self.x *= rhs;
//         self.y *= rhs;
//     }
// }
//
// impl std::ops::Div<f32> for Vec2 {
//     type Output = Vec2;
//
//     fn div(self, rhs: f32) -> Self::Output {
//         Vec2 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }
// impl std::ops::Div<f32> for &Vec2 {
//     type Output = Vec2;
//
//     fn div(self, rhs: f32) -> Self::Output {
//         Vec2 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }
// impl std::ops::DivAssign<f32> for Vec2 {
//     fn div_assign(&mut self, rhs: f32) {
//         self.x /= rhs;
//         self.y /= rhs;
//     }
// }
