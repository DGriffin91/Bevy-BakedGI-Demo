use bevy::prelude::*;

mod custom_material;
mod emissive_material;
mod level1;
mod level2;
mod planets;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use custom_material::{CustomMaterial, MaterialProperties};
use emissive_material::EmissiveMaterial;
use planets::{planitary_physics, spawn_planets};

#[derive(Component, Debug)]
struct LevelAsset {
    pub material_properties: MaterialProperties,
    pub material_handle: Handle<CustomMaterial>,
}

fn menu_ui(
    mut windows: ResMut<Windows>,
    mut egui_context: ResMut<EguiContext>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut level_asset_query: Query<&mut LevelAsset>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary_mut().unwrap();
    if window.is_focused() && !window.cursor_locked() {
        egui::Window::new("materials").show(egui_context.ctx_mut(), |ui| {
            let mut level_assets = level_asset_query.iter_mut();
            let mut main = level_assets.next().unwrap();
            ui.collapsing("material properties", |ui| {
                main.material_properties.build_ui(ui);
            });
            for (i, mut level_asset) in level_assets.enumerate() {
                level_asset.material_properties = main.material_properties;
                if let Some(mat) = custom_materials.get_mut(&level_asset.material_handle) {
                    mat.material_properties = main.material_properties;
                    ui.collapsing(format!("level asset {}", i), |ui| {
                        mat.build_ui(ui, &asset_server);
                    });
                }
            }
        });
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00002,
            speed: 48.,
        })
        .add_startup_system(level1::setup_room)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<EmissiveMaterial>::default())
        .add_system(menu_ui)
        .add_startup_system(spawn_planets)
        .add_system(planitary_physics)
        .run();
}
