use bevy::{prelude::*, window::CursorGrabMode};

mod custom_material;
mod emissive_material;
mod level1;
mod level2;
mod planets;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use custom_material::{set_texture_settings, CustomMaterial};
use emissive_material::EmissiveMaterial;
use planets::{planitary_physics, spawn_planets};

#[derive(Component)]
pub struct LevelItem;

#[allow(clippy::too_many_arguments)]
fn menu_ui(
    mut com: Commands,
    mut windows: ResMut<Windows>,
    mut egui_context: ResMut<EguiContext>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut emissive_materials: ResMut<Assets<EmissiveMaterial>>,
    mut material_handles: Query<&mut Handle<CustomMaterial>>,
    level_items: Query<Entity, With<LevelItem>>,
    asset_server: Res<AssetServer>,
    mut controllers: Query<&mut CameraController>,
) {
    let window = windows.get_primary_mut().unwrap();
    let show_ui = window.is_focused() && !(window.cursor_grab_mode() == CursorGrabMode::Locked);
    if show_ui {
        egui::Window::new("Settings").show(egui_context.ctx_mut(), |ui| {
            if ui.button("Load Level 1").clicked() {
                for entity in level_items.iter() {
                    com.entity(entity).despawn_recursive();
                }
                level1::setup_room(
                    &mut com,
                    &mut custom_materials,
                    &mut emissive_materials,
                    &asset_server,
                );
            }
            if ui.button("Load Level 2").clicked() {
                for entity in level_items.iter() {
                    com.entity(entity).despawn_recursive();
                }
                level2::setup_room(
                    &mut com,
                    &mut custom_materials,
                    &mut emissive_materials,
                    &asset_server,
                );
            }
            if let Some(handle) = material_handles.iter_mut().next() {
                let main_mat = if let Some(main_mat) = custom_materials.get_mut(&handle.clone()) {
                    ui.collapsing("material properties", |ui| {
                        main_mat.build_ui(ui, &mut com, &asset_server);
                    });
                    Some(main_mat.clone())
                } else {
                    None
                };
                if let Some(main_mat) = main_mat {
                    for handle in material_handles.iter_mut() {
                        if let Some(mat) = custom_materials.get_mut(&handle.clone()) {
                            mat.material_properties = main_mat.material_properties
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

fn player(mut com: Commands) {
    // camera
    com.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(CameraController::default().print_controls());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<EmissiveMaterial>::default())
        .add_system(menu_ui)
        .add_startup_system(spawn_planets)
        .add_startup_system(player)
        .add_system(planitary_physics)
        .add_system(set_texture_settings)
        .run();
}
