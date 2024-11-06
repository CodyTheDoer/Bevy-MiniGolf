use bevy::prelude::*;

use crate::CameraWorld;

pub fn setup_3d_camera(
    mut commands: Commands,
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn((
        camera,
        CameraWorld,
    ));
}
