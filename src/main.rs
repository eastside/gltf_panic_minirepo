use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(AmbientLight {
        brightness: 1.0,
        ..default()
    })
    .add_systems(Startup, setup)
    .run()
    ;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn the first scene in `models/SimpleSkin/SimpleSkin.gltf`
    commands.spawn(SceneBundle {
        scene: asset_server.load("SyntyCharacterWithAnim.gltf#Scene0"),
        ..default()
    });
}