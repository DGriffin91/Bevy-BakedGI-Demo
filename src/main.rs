use bevy::prelude::*;

mod custom_material;
mod emissive_material;
mod level1;
mod level2;
mod planets;
mod player_plugin;
use player_plugin::{MovementSettings, PlayerPlugin};
use custom_material::{CustomMaterial, MaterialProperties};
use emissive_material::EmissiveMaterial;
use planets::{planitary_physics, spawn_planets};

#[derive(Component, Debug)]
struct LevelAsset {
    pub material_properties: MaterialProperties,
    pub material_handle: Handle<CustomMaterial>,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings::default())
        .add_startup_system(level1::setup_room)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<EmissiveMaterial>::default())
        .add_startup_system(spawn_planets)
        .add_system(planitary_physics)
        .run();
}
