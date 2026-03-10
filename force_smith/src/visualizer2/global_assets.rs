use bevy::{platform::collections::HashMap, prelude::*};

pub struct GlobalAssetPlugin;
impl Plugin for GlobalAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_global_color_asset, add_global_shape_asset));
    }
}

#[derive(Resource)]
pub struct GlobalShapeAssets {
    pub unit_rectangle: Handle<Mesh>,
    pub unit_circle: Handle<Mesh>,
    pub unit_triangle: Handle<Mesh>,
}

fn add_global_shape_asset(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(GlobalShapeAssets {
        unit_rectangle: meshes.add(Rectangle::new(1.0, 1.0)),
        unit_circle: meshes.add(Circle::new(1.0)),
        unit_triangle: meshes.add(Triangle2d::new(
            Vec2::new(1.0, 0.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
        )),
    });
}

fn add_global_color_asset(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let color_map: HashMap<GlobalColor, Handle<ColorMaterial>> = GlobalColor::ALL
        .into_iter()
        .map(|c| (c, materials.add(c.to_color())))
        .collect();
    commands.insert_resource(GlobalColorAsset::from(color_map));
}

#[derive(Resource, Deref)]
pub struct GlobalColorAsset(HashMap<GlobalColor, Handle<ColorMaterial>>);
impl From<HashMap<GlobalColor, Handle<ColorMaterial>>> for GlobalColorAsset {
    fn from(value: HashMap<GlobalColor, Handle<ColorMaterial>>) -> Self {
        Self(value)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum GlobalColor {
    Red,
    Green,
    Blue,
}
impl GlobalColor {
    const ALL: [Self; 3] = [Self::Red, Self::Green, Self::Blue];
    pub fn to_color(self) -> Color {
        match self {
            GlobalColor::Red => Color::srgb(1.0, 0.0, 0.0),
            GlobalColor::Green => Color::srgb(0.0, 1.0, 0.0),
            GlobalColor::Blue => Color::srgb(0.0, 0.0, 1.0),
        }
    }
}
