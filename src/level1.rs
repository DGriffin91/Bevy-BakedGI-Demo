use bevy::prelude::*;

use crate::custom_material::{load_mark, CustomMaterial, MaterialProperties, MaterialSetProp};
use crate::emissive_material::EmissiveMaterial;

pub fn setup_room(
    com: &mut Commands,
    custom_materials: &mut Assets<CustomMaterial>,
    emissive_materials: &mut Assets<EmissiveMaterial>,
    ass: &Res<AssetServer>,
) {
    //Building Objects
    let building_objects = ass.load("models/scene1/building.glb#Mesh0/Primitive0");

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

    let mut material = CustomMaterial {
        material_properties,
        lightmap: Some(ass.load("textures/scene1/objects_lightmap.jpg")),
        lightmap_path: String::from("textures/scene1/objects_lightmap.jpg"),
        base: Some(load_mark(com, &ass, "textures/concrete.jpg")),
        base_path: String::from("textures/concrete.jpg"),
        vary: Some(load_mark(com, &ass, "textures/detail.jpg")),
        vary_path: String::from("textures/detail.jpg"),
        reflection: Some(load_mark(com, &ass, "textures/scene1/reflection.jpg")),
        reflection_path: String::from("textures/scene1/reflection.jpg"),
        walls: Some(load_mark(com, &ass, "textures/concrete3.jpg")),
        walls_path: String::from("textures/concrete3.jpg"),
    };

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: custom_materials.add(material.clone()),
        ..Default::default()
    });

    //Building Main
    let building_main = ass.load("models/scene1/building.glb#Mesh1/Primitive0");

    material.lightmap = Some(ass.load("textures/scene1/main_lightmap.jpg"));
    material.lightmap_path = String::from("textures/scene1/main_lightmap.jpg");

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_main,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: custom_materials.add(material),
        ..Default::default()
    });

    //Sky Box
    let skybox_texture = ass.load("textures/scene1/skybox.jpg");
    let skybox = ass.load("models/scene1/skybox.glb#Mesh0/Primitive0");
    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: skybox,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10.0, 10.0, 10.0)),
        material: emissive_materials.add(EmissiveMaterial {
            emissive: Color::WHITE,
            emissive_texture: Some(skybox_texture),
        }),
        ..Default::default()
    });

    //Bevy Sun
    let size: f32 = 50.0;
    com.spawn_bundle(DirectionalLightBundle {
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
    com.spawn_bundle(PointLightBundle {
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
        com.spawn_bundle(PointLightBundle {
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
    ass.watch_for_changes().unwrap();
}
