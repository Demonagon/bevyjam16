use bevy::prelude::*;

#[derive(Resource)]
pub struct Meshes {
    pub circle: Handle<Mesh>,
    pub square: Handle<Mesh>,
}

#[derive(Resource)]
pub struct Colors {
    pub black: Handle<ColorMaterial>,
    pub gray: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
}

impl FromWorld for Meshes {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();

        Meshes {
            circle: meshes.add(Circle::new(1.0)),
            square: meshes.add(Rectangle::new(1.0, 1.0)),
        }
    }
}

impl Meshes {
    pub fn circle(&self) -> impl Bundle {
        Mesh2d(self.circle.clone())
    }
    pub fn square(&self) -> impl Bundle {
        Mesh2d(self.square.clone())
    }
}

impl FromWorld for Colors {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<ColorMaterial>>();

        Colors {
            black: mats.add(Color::BLACK),
            gray: mats.add(Color::linear_rgb(0.5, 0.5, 0.5)),
            white: mats.add(Color::WHITE),
        }
    }
}

impl Colors {
    pub fn white(&self) -> impl Bundle {
        MeshMaterial2d(self.white.clone())
    }
    pub fn gray(&self) -> impl Bundle {
        MeshMaterial2d(self.gray.clone())
    }
    pub fn black(&self) -> impl Bundle {
        MeshMaterial2d(self.black.clone())
    }
}
