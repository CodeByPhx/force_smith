use bevy::prelude::*;

pub type GraphEntitySelector = Or<(With<NodeMarker>, With<EdgeMarker>)>;

#[derive(Bundle)]
pub struct NodeBundle {
    pub marker: NodeMarker,
    pub idx: Index,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}
impl NodeBundle {
    pub fn new(
        idx: usize,
        position: Vec3,
        radius: f32,
        unit_mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            marker: NodeMarker,
            idx: idx.into(),
            mesh: Mesh2d(unit_mesh),
            material: MeshMaterial2d(material),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(radius),
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct NodeMarker;

#[derive(Component, Deref)]
pub struct Index(usize);
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Bundle)]
pub struct EdgeBundle {
    pub marker: EdgeMarker,
    pub edge_indices: EdgeIndices,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}
impl EdgeBundle {
    pub fn new(
        from_idx: usize,
        to_idx: usize,
        from_position: Vec3,
        to_position: Vec3,
        width: f32,
        unit_mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        let direction = to_position - from_position;
        let midpoint = from_position + direction / 2.0;
        let angle = direction.truncate().to_angle();

        Self {
            marker: EdgeMarker,
            edge_indices: (from_idx, to_idx).into(),
            mesh: Mesh2d(unit_mesh),
            material: MeshMaterial2d(material),
            transform: Transform {
                translation: midpoint,
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(direction.length(), width, 1.0),
            },
        }
    }
}

#[derive(Component)]
pub struct EdgeMarker;

#[derive(Component)]
pub struct EdgeIndices {
    pub from: usize,
    pub to: usize,
}
impl From<(usize, usize)> for EdgeIndices {
    fn from(value: (usize, usize)) -> Self {
        Self {
            from: value.0,
            to: value.1,
        }
    }
}

#[derive(Component)]
pub struct ArrowMarker;

pub struct ArrowShaftBundle {
    pub marker: ArrowMarker,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}
impl ArrowShaftBundle {
    pub fn new(
        from_position: Vec3,
        to_position: Vec3,
        width: f32,
        unit_mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        let direction = to_position - from_position;
        let midpoint = from_position + direction / 2.0;
        let angle = direction.truncate().to_angle();

        Self {
            marker: ArrowMarker,
            mesh: Mesh2d(unit_mesh),
            material: MeshMaterial2d(material),
            transform: Transform {
                translation: midpoint,
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(direction.length(), width, 1.0),
            },
        }
    }
}

pub struct ArrowTipBundle {
    pub marker: ArrowMarker,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}
impl ArrowTipBundle {
    pub fn new(
        from_position: Vec3,
        to_position: Vec3,
        width: f32,
        unit_mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        let from_position2d = from_position.truncate();
        let to_position2d = to_position.truncate();

        let direction2d = to_position2d - from_position2d;
        let angle = direction2d.to_angle();

        Self {
            marker: ArrowMarker,
            mesh: Mesh2d(unit_mesh),
            material: MeshMaterial2d(material),
            transform: Transform {
                translation: Vec3::new(to_position.x, to_position.y, from_position.z),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(direction2d.length(), width / 2.0, 1.0),
            },
        }
    }
}

pub fn calculate_unit_rectangle_transform(
    from_position: Vec3,
    to_position: Vec3,
    width: f32,
) -> Transform {
    let direction = to_position - from_position;
    let midpoint = from_position + direction / 2.0;
    let angle = direction.truncate().to_angle();
    Transform {
        translation: midpoint,
        rotation: Quat::from_rotation_z(angle),
        scale: Vec3::new(direction.length(), width, 1.0),
    }
}
