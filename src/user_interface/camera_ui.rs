use bevy::prelude::*;

use crate::{CameraUi, Fonts};

pub fn setup_ui_camera(
    mut commands: Commands,
    fonts: Res<Fonts>
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: -1, // Render on top of the 3D scene
                ..default()
                },
            ..default()
        },
        CameraUi,
    ));

    // Create a screen-sized UI node as a container
    commands.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,    // Center vertically within the container
            justify_content: JustifyContent::Center, // Center horizontally within the container
            position_type: PositionType::Absolute,
            // Set this node to occupy the entire screen
            width: Val::Percent(100.0),   // Use width instead of size
            height: Val::Percent(100.0),  // Use height instead of size
            ..default()
        },
        ..default()
    });

    info!("{:?}", fonts.fonts);
    // .with_children(|parent| {
    //     parent.spawn(TextBundle {
    //         text: Text {
    //             sections: vec![TextSection::new(
    //                 "Mini Golf",
    //                 fonts.fonts.matrix_display.clone(),
    //             )],
    //             ..default()
    //         },
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             top: Val::Percent(2.0), 
    //             ..default()
    //         },
    //         ..default()
    //     });
    // });
}
