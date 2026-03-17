use bevy_math::Vec2;

pub struct SafeVec2<'a>(&'a Vec2);
impl<'a> From<&'a Vec2> for SafeVec2<'a> {
    fn from(value: &'a Vec2) -> Self {
        Self(value)
    }
}
impl<'a> From<SafeVec2<'a>> for Vec2 {
    fn from(value: SafeVec2<'a>) -> Self {
        *value.0
    }
}

pub struct SafeMutVec2<'a>(&'a mut Vec2);
impl<'a> From<&'a mut Vec2> for SafeMutVec2<'a> {
    fn from(value: &'a mut Vec2) -> Self {
        Self(value)
    }
}
impl<'a> From<SafeMutVec2<'a>> for Vec2 {
    fn from(value: SafeMutVec2<'a>) -> Self {
        *value.0
    }
}
