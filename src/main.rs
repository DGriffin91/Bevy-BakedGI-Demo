use bevy::prelude::*;

mod custom_material;
mod emissive_material;
mod planets;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{MovementSettings, PlayerPlugin};
use custom_material::{CustomMaterial, MaterialProperties, MaterialSetProp, MaterialTexture};
use emissive_material::EmissiveMaterial;
use planets::{planitary_physics, spawn_planets};

#[derive(Component, Debug)]
struct LevelAsset {
    pub material_properties: MaterialProperties,
    pub material_handle: Handle<CustomMaterial>,
}

fn setup_room(
    mut commands: Commands,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut emissive_materials: ResMut<Assets<EmissiveMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let variation_texture =
        MaterialTexture::new(&asset_server, "textures/detail.jpg", "variation_texture");
    let base_texture = MaterialTexture::new(&asset_server, "textures/concrete.jpg", "base_texture");

    let walls_texture =
        MaterialTexture::new(&asset_server, "textures/concrete3.jpg", "walls_texture");

    let reflection_texture = MaterialTexture::new(
        &asset_server,
        "textures/reflection.jpg",
        "reflection_texture",
    );

    //Building Objects
    let objects_lightmap = MaterialTexture::new(
        &asset_server,
        "textures/objects_lightmap.jpg",
        "objects_lightmap",
    );
    let building_objects = asset_server.load("models/building.glb#Mesh0/Primitive0");

    let material_properties = MaterialProperties {
        lightmap: MaterialSetProp {
            scale: 1.0,
            contrast: 1.8,
            brightness: 3.1,
            blend: 1.0,
        },
        base_a: MaterialSetProp {
            scale: 8.5,
            contrast: 0.33,
            brightness: 2.0,
            blend: 1.0,
        },
        base_b: MaterialSetProp {
            scale: 30.0,
            contrast: 0.3,
            brightness: 2.2,
            blend: 1.0,
        },
        vary_a: MaterialSetProp {
            scale: 0.14,
            contrast: 0.77,
            brightness: 4.2,
            blend: 0.057,
        },
        vary_b: MaterialSetProp {
            scale: 5.0,
            contrast: 0.14,
            brightness: 1.05,
            blend: 1.0,
        },
        reflection: MaterialSetProp {
            scale: 1.0,
            contrast: 3.0,
            brightness: 0.115,
            blend: 1.0,
        },
        walls: MaterialSetProp {
            scale: 10.5,
            contrast: 0.53,
            brightness: 1.6,
            blend: 1.0,
        },
        reflection_mask: MaterialSetProp {
            scale: 0.033,
            contrast: 2.3,
            brightness: 40.0,
            blend: 1.0,
        },
        mist: MaterialSetProp {
            scale: 0.032,
            contrast: 1.0,
            brightness: 1.0,
            blend: 0.567,
        },
        directional_light_blend: 0.6,
    };

    let material = custom_materials.add(CustomMaterial {
        material_properties,
        textures: [
            objects_lightmap,
            base_texture.clone(),
            variation_texture.clone(),
            reflection_texture.clone(),
            walls_texture.clone(),
        ],
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: building_objects,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: material.clone(),
            ..Default::default()
        })
        .insert(LevelAsset {
            material_properties,
            material_handle: material,
        });

    //Building Main
    let main_lightmap =
        MaterialTexture::new(&asset_server, "textures/main_lightmap.jpg", "main_lightmap");
    let building_main = asset_server.load("models/building.glb#Mesh1/Primitive0");

    let material = custom_materials.add(CustomMaterial {
        material_properties,
        textures: [
            main_lightmap,
            base_texture.clone(),
            variation_texture.clone(),
            reflection_texture,
            walls_texture,
        ],
    });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: building_main,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: material.clone(),
            ..Default::default()
        })
        .insert(LevelAsset {
            material_properties,
            material_handle: material,
        });

    //Sky Box
    let sky_box_texture = asset_server.load("textures/sky_box.jpg");
    let skybox = asset_server.load("models/skybox.glb#Mesh0/Primitive0");
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: skybox,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10.0, 10.0, 10.0)),
        material: emissive_materials.add(EmissiveMaterial {
            emissive: Color::WHITE,
            emissive_texture: Some(sky_box_texture),
        }),
        ..Default::default()
    });

    //Bevy Sun
    let size: f32 = 50.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -size * 4.0,
                right: size * 2.0,
                bottom: -size * 2.0,
                top: size * 1.0,
                near: -size * 2.0,
                far: size * 1.0,
                ..Default::default()
            },
            illuminance: 100000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                (-14.0f32).to_radians(),
                -(192.0 - 180.0f32).to_radians(),
                0.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    //Sky Light for PBR
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 100.0),
        point_light: PointLight {
            intensity: 30000.0,
            range: 1000.0,
            radius: 100.0,
            color: Color::rgb(0.5, 0.45, 1.0),
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });

    // Only doing a couple light positions because Bevy complains:
    // WARN bevy_pbr::render::light: Cluster light index lists is full!
    // The PointLights in the view are affecting too many clusters.
    let lamp_locations = [
        //Vec3::new(-15.0, 17.0, -16.0),
        Vec3::new(-10.0, 17.0, -16.0),
        //Vec3::new(-10.0, 17.0, -16.0),
        //Vec3::new(-5.0, 17.0, -16.0),
        //Vec3::new(-5.0, 17.0, -16.0),
        //Vec3::new(0.0, 17.0, -16.0),
        //Vec3::new(5.0, 17.0, -16.0),
        Vec3::new(10.0, 17.0, -16.0),
        //Vec3::new(15.0, 17.0, -16.0),
    ];

    for lamp_loc in lamp_locations {
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(lamp_loc.x, lamp_loc.y, lamp_loc.z),
            point_light: PointLight {
                intensity: 500.0,
                range: 1000.0,
                radius: 10.0, //Oversize since we only have 2
                color: Color::rgb(1.0, 1.0, 1.0),
                shadows_enabled: false,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // Tell the asset server to watch for asset changes on disk:
    asset_server.watch_for_changes().unwrap();
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
            let mut it = level_asset_query.iter_mut();
            let mut main = it.next().unwrap();
            let mut walls = it.next().unwrap();
            ui.collapsing("material properties", |ui| {
                main.material_properties.build_ui(ui);
            });
            walls.material_properties = main.material_properties;
            if let Some(mat) = custom_materials.get_mut(&main.material_handle) {
                mat.material_properties = main.material_properties;
                ui.collapsing("main material", |ui| {
                    mat.build_ui(ui, &asset_server);
                });
            }
            if let Some(mat) = custom_materials.get_mut(&walls.material_handle) {
                mat.material_properties = main.material_properties;
                ui.collapsing("walls material", |ui| {
                    mat.build_ui(ui, &asset_server);
                });
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
        .insert_resource(MovementSettings::default())
        .add_startup_system(setup_room)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<EmissiveMaterial>::default())
        .add_system(menu_ui)
        .add_startup_system(spawn_planets)
        .add_system(planitary_physics)
        .run();
}
