use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};

use rand::{thread_rng, Rng};

mod camera;

use camera::IsoCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(IsoCameraPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let grass_texture_handle = asset_server.load("textures/grass.png");
    let dirt_texture_handle = asset_server.load("textures/dirt.png");
    let sand_texture_handle = asset_server.load("textures/sand.png");
    let stone_texture_handle = asset_server.load("textures/stone.png");

    let grass_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(grass_texture_handle.clone()),
        perceptual_roughness: 1.0,
        ..default()
    });

    let dirt_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(dirt_texture_handle.clone()),
        perceptual_roughness: 1.0,
        ..default()
    });

    let sand_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(sand_texture_handle.clone()),
        perceptual_roughness: 1.0,
        ..default()
    });

    let stone_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(stone_texture_handle.clone()),
        perceptual_roughness: 1.0,
        ..default()
    });

    let blocks = [grass_material_handle.clone(), dirt_material_handle.clone(), sand_material_handle.clone(), stone_material_handle.clone()];

    let block_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    let translation = Vec3::new(5.0, 7.0, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 10.0,
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }
            .into(),
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera::IsoCamera {
            radius,
            ..Default::default()
        },
    ));

    let mut rng = thread_rng();

    for y in -16..16 {
        for x in -16..16 {
            commands.spawn(PbrBundle {
                mesh: block_handle.clone(),
                material: blocks[rng.gen_range(0..4)].clone(),
                transform: Transform::from_xyz(x as f32, -1.0, y as f32),
                ..default()
            });
        }
    }

    commands.spawn((
        PbrBundle {
            mesh: block_handle.clone(),
            material: grass_material_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        camera::Player {
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}