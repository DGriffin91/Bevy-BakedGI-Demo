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
        "textures/scene1/objects_lightmap.jpg",
        "objects_lightmap",
    );
    let building_objects = asset_server.load("models/scene1/building.glb#Mesh0/Primitive0");

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

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: building_objects,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: material.clone(),
        ..Default::default()
    });

    //Building Main
    let main_lightmap = MaterialTexture::new(
        &asset_server,
        "textures/scene1/main_lightmap.jpg",
        "main_lightmap",
    );
    let building_main = asset_server.load("models/scene1/building.glb#Mesh1/Primitive0");

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
    let skybox_texture = asset_server.load("textures/scene1/skybox.jpg");
    let skybox = asset_server.load("models/scene1/skybox.glb#Mesh0/Primitive0");
    commands.spawn().insert_bundle(MaterialMeshBundle {
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
