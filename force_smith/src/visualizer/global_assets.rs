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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GlobalColor {
    // Primary colors (softer, more muted)
    Red,
    Green,
    Blue,

    // Accent colors
    Orange,
    Purple,
    Cyan,
    Pink,
    Yellow,

    // Neutrals
    White,
    LightGrey,
    Grey,
    DarkGrey,
    Black,
}
impl GlobalColor {
    const ALL: [Self; 13] = [
        Self::Red,
        Self::Green,
        Self::Blue,
        Self::Orange,
        Self::Purple,
        Self::Cyan,
        Self::Pink,
        Self::Yellow,
        Self::White,
        Self::LightGrey,
        Self::Grey,
        Self::DarkGrey,
        Self::Black,
    ];
    pub fn to_color(self) -> Color {
        match self {
            // Softer, more muted primary colors
            GlobalColor::Red => Color::srgb(0.85, 0.35, 0.35),        // Soft red
            GlobalColor::Green => Color::srgb(0.4, 0.85, 0.5),        // Soft green
            GlobalColor::Blue => Color::srgb(0.4, 0.65, 0.95),        // Soft blue

            // Accent colors
            GlobalColor::Orange => Color::srgb(0.95, 0.65, 0.35),     // Warm orange
            GlobalColor::Purple => Color::srgb(0.75, 0.5, 0.95),      // Soft purple
            GlobalColor::Cyan => Color::srgb(0.4, 0.85, 0.85),        // Bright cyan
            GlobalColor::Pink => Color::srgb(0.95, 0.55, 0.75),       // Soft pink
            GlobalColor::Yellow => Color::srgb(0.95, 0.9, 0.45),      // Warm yellow

            // Neutrals
            GlobalColor::White => Color::srgb(0.95, 0.95, 0.95),      // Off-white
            GlobalColor::LightGrey => Color::srgb(0.75, 0.75, 0.75),  // Light grey
            GlobalColor::Grey => Color::srgb(0.5, 0.5, 0.5),          // Medium grey
            GlobalColor::DarkGrey => Color::srgb(0.25, 0.25, 0.25),   // Dark grey
            GlobalColor::Black => Color::srgb(0.1, 0.1, 0.1),         // Near-black
        }
    }
}
