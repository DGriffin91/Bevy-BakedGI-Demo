use bevy::prelude::*;

use crate::custom_material::{
    CustomMaterial, MaterialProperties, MaterialSetProp, MaterialTexture,
};
use crate::emissive_material::EmissiveMaterial;

pub fn setup_room(
    commands: &mut Commands,
    custom_materials: &mut Assets<CustomMaterial>,
    emissive_materials: &mut Assets<EmissiveMaterial>,
    asset_server: &Res<AssetServer>,
) {
    let variation_texture =
        MaterialTexture::new(&asset_server, "textures/detail.jpg", "variation_texture");
    let base_texture = MaterialTexture::new(&asset_server, "textures/concrete.jpg", "base_texture");

    let walls_texture =
        MaterialTexture::new(&asset_server, "textures/concrete3.jpg", "walls_texture");

    let reflection_texture = MaterialTexture::new(
        &asset_server,
        "textures/scene1/reflection.jpg",
        "reflection_texture",
    );

    //Building Objects
    let objects_lightmap = MaterialTexture::new(
        &asset_server,
        "textures/scene2/objects_lightmap.jpg",
        "objects_lightmap",
    );

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

    let building_objects = asset_server.load("models/scene2/building.glb#Mesh0/Primitive0");

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material.clone(),
        ..Default::default()
    });

    let building_objects = asset_server.load("models/scene2/building.glb#Mesh2/Primitive0");

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material.clone(),
        ..Default::default()
    });

    let building_objects = asset_server.load("models/scene2/building.glb#Mesh3/Primitive0");

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material.clone(),
        ..Default::default()
    });

    //Building Main
    let main_lightmap = MaterialTexture::new(
        &asset_server,
        "textures/scene2/walls_lightmap.jpg",
        "main_lightmap",
    );
    let building_main = asset_server.load("models/scene2/building.glb#Mesh1/Primitive0");

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

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_main,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material.clone(),
        ..Default::default()
    });

    //Sky Box
    let skybox_texture = asset_server.load("textures/scene2/skybox.jpg");
    let skybox = asset_server.load("models/scene2/skybox.glb#Mesh0/Primitive0");
    commands.spawn().insert_bundle(MaterialMeshBundle {
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
    //commands.spawn_bundle(DirectionalLightBundle {
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
    commands.spawn_bundle(PointLightBundle {
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
    asset_server.watch_for_changes().unwrap();
}
