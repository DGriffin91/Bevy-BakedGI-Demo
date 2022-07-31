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
    let material_properties = MaterialProperties {
        lightmap: MaterialSetProp {
            scale: 1.0,
            contrast: 2.8,
            brightness: 0.58,
            blend: 1.0,
        },
        base_a: MaterialSetProp {
            scale: 12.5,
            contrast: 0.215,
            brightness: 1.8,
            blend: 1.0,
        },
        base_b: MaterialSetProp {
            scale: 52.0,
            contrast: 0.16,
            brightness: 1.5,
            blend: 1.0,
        },
        vary_a: MaterialSetProp {
            scale: 0.52,
            contrast: 0.83,
            brightness: 4.2,
            blend: 0.072,
        },
        vary_b: MaterialSetProp {
            scale: 9.5,
            contrast: 0.165,
            brightness: 1.65,
            blend: 0.55,
        },
        reflection: MaterialSetProp {
            scale: 1.0,
            contrast: 5.0,
            brightness: 0.53,
            blend: 0.73,
        },
        walls: MaterialSetProp {
            scale: 10.5,
            contrast: 0.53,
            brightness: 1.6,
            blend: 1.0,
        },
        reflection_mask: MaterialSetProp {
            scale: 0.053,
            contrast: 2.3,
            brightness: 40.0,
            blend: 1.0,
        },
        mist: MaterialSetProp {
            scale: 0.021,
            contrast: 1.7,
            brightness: 17.0,
            blend: 0.78,
        },
        directional_light_blend: 0.6,
    };

    let mut material = CustomMaterial {
        material_properties,
        lightmap: Some(ass.load("textures/scene2/objects_lightmap.jpg")),
        lightmap_path: String::from("textures/scene2/objects_lightmap.jpg"),
        base: Some(load_mark(com, &ass, "textures/concrete.jpg")),
        base_path: String::from("textures/concrete.jpg"),
        vary: Some(load_mark(com, &ass, "textures/detail.jpg")),
        vary_path: String::from("textures/detail.jpg"),
        reflection: Some(load_mark(com, &ass, "textures/scene1/reflection.jpg")),
        reflection_path: String::from("textures/scene1/reflection.jpg"),
        walls: Some(load_mark(com, &ass, "textures/concrete3.jpg")),
        walls_path: String::from("textures/concrete3.jpg"),
    };

    let material_handle = custom_materials.add(material.clone());

    let building_objects = ass.load("models/scene2/building.glb#Mesh0/Primitive0");

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material_handle.clone(),
        ..Default::default()
    });

    let building_objects = ass.load("models/scene2/building.glb#Mesh2/Primitive0");

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material_handle.clone(),
        ..Default::default()
    });

    let building_objects = ass.load("models/scene2/building.glb#Mesh3/Primitive0");

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material_handle,
        ..Default::default()
    });

    //Building Main
    let building_main = ass.load("models/scene2/building.glb#Mesh1/Primitive0");

    material.lightmap = Some(ass.load("textures/scene2/walls_lightmap.jpg"));
    material.lightmap_path = String::from("textures/scene2/walls_lightmap.jpg");

    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_main,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: custom_materials.add(material),
        ..Default::default()
    });

    //Sky Box
    let skybox_texture = ass.load("textures/scene2/skybox.jpg");
    let skybox = ass.load("models/scene2/skybox.glb#Mesh0/Primitive0");
    com.spawn().insert_bundle(MaterialMeshBundle {
        mesh: skybox,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(10.0, 10.0, 10.0)),
        material: emissive_materials.add(EmissiveMaterial {
            emissive: Color::WHITE,
            emissive_texture: Some(skybox_texture),
        }),
        ..Default::default()
    });

    //let blender_sun_elev = 24.4f32;
    //let blender_sun_rot = 248.0f32;
    ////Bevy Sun
    //let size: f32 = 250.0;
    //com.spawn_bundle(DirectionalLightBundle {
    //    directional_light: DirectionalLight {
    //        // Configure the projection to better fit the scene
    //        shadow_projection: OrthographicProjection {
    //            left: -size * 4.0,
    //            right: size * 2.0,
    //            bottom: -size * 2.0,
    //            top: size * 1.0,
    //            near: -size * 2.0,
    //            far: size * 1.0,
    //            ..Default::default()
    //        },
    //        illuminance: 100000.0,
    //        shadows_enabled: true,
    //        ..Default::default()
    //    },
    //    transform: Transform {
    //        translation: Vec3::new(0.0, 0.0, 0.0),
    //        rotation: Quat::from_euler(
    //            EulerRot::XYZ,
    //            (-blender_sun_elev).to_radians(),
    //            -(blender_sun_rot - 180.0f32).to_radians(),
    //            0.0,
    //        ),
    //        ..Default::default()
    //    },
    //    ..Default::default()
    //});

    //Sky Light for PBR
    com.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
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

    // Tell the asset server to watch for asset changes on disk:
    ass.watch_for_changes().unwrap();
}
