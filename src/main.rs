use bevy::prelude::*;

mod custom_material;
mod emissive_material;
mod level1;
mod level2;
mod planets;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use custom_material::CustomMaterial;
use emissive_material::EmissiveMaterial;
use planets::{planitary_physics, spawn_planets};
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

fn menu_ui(
    mut commands: Commands,
    query: Query<Entity>,
    mut windows: ResMut<Windows>,
    mut egui_context: ResMut<EguiContext>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut emissive_materials: ResMut<Assets<EmissiveMaterial>>,
    mut material_handles: Query<&mut Handle<CustomMaterial>>,
    asset_server: Res<AssetServer>,
    mut controllers: Query<&mut UnrealCameraController>,
) {
    let window = windows.get_primary_mut().unwrap();
    let show_ui = window.is_focused() && !window.cursor_locked();
    if show_ui {
        egui::Window::new("materials").show(egui_context.ctx_mut(), |ui| {
            if ui.button("Load Level 1").clicked() {
                for entity in query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                player(&mut commands);
                level1::setup_room(
                    &mut commands,
                    &mut custom_materials,
                    &mut emissive_materials,
                    &asset_server,
                );
            }
            if ui.button("Load Level 2").clicked() {
                for entity in query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                player(&mut commands);
                level2::setup_room(
                    &mut commands,
                    &mut custom_materials,
                    &mut emissive_materials,
                    &asset_server,
                );
            }
            if let Some(handle) = material_handles.iter_mut().next() {
                let main_mat = if let Some(main_mat) = custom_materials.get_mut(&handle.clone()) {
                    ui.collapsing("material properties", |ui| {
                        main_mat.build_ui(ui, &asset_server);
                    });
                    Some(main_mat.clone())
                } else {
                    None
                };
                if let Some(main_mat) = main_mat {
                    for handle in material_handles.iter_mut() {
                        if let Some(mat) = custom_materials.get_mut(&handle.clone()) {
                            mat.material_properties = main_mat.material_properties.clone()
                        }
                    }
                }
            }
            if let Some(mut controller) = controllers.iter_mut().next() {
                controller.enabled = !ui.ctx().is_using_pointer();
            }
        });
    }
}

fn player(commands: &mut Commands) {
    commands.spawn_bundle(UnrealCameraBundle::new(
        UnrealCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-30.0, 3.0, -3.0),
        Vec3::new(0.0, 3.0, -3.0),
    ));
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<EmissiveMaterial>::default())
        .add_system(menu_ui)
        .add_startup_system(spawn_planets)
        .add_system(planitary_physics)
        .run();
}
