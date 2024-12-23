use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI, TAU};

// State
use crate::{
    // StateCameraMenuTarget, 
    StateCameraOrbitEntity, 
    StatePanOrbit,
};

// Resource
use crate::{
    CameraHandler, 
    CameraWorld,
    GolfBall,
    PanOrbitAction,
    PanOrbitCameraBundle,
    PanOrbitSettings,
    Party,
    RunTrigger,
};

impl CameraHandler {
    pub fn new() -> Self {
        let current_coords: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        CameraHandler {
            current_coords,
        }
    }
}

impl Default for StatePanOrbit {
    fn default() -> Self {
        StatePanOrbit {
            center: Vec3::ZERO,
            radius: 1.0,
            upside_down: false,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.1f32.to_radians(), // 0.1 degree per pixel
            zoom_sensitivity: 0.01,
            pan_key: Some(KeyCode::ControlLeft),
            orbit_key: Some(KeyCode::AltLeft),
            zoom_key: Some(KeyCode::ShiftLeft),
            scroll_action: Some(PanOrbitAction::Zoom),
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

pub fn camera_handler_cycle_state_camera(
    mut run_trigger: ResMut<RunTrigger>,
    camera_orbit_entity_state: Res<State<StateCameraOrbitEntity>>,
    mut next_camera_orbit_entity_state: ResMut<NextState<StateCameraOrbitEntity>>,
) {
    info!("function: camera_handler_cycle_state_camera"); 
    match camera_orbit_entity_state.get() {
        StateCameraOrbitEntity::Menu => {
            info!("StateCameraOrbitEntity::Ball");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::Ball);
        },
        StateCameraOrbitEntity::Ball => {
            info!("StateCameraOrbitEntity::Cup");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::Cup);
        },
        StateCameraOrbitEntity::Cup => {
            info!("StateCameraOrbitEntity::FreePan");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::FreePan);
        },
        StateCameraOrbitEntity::FreePan => {
            info!("StateCameraOrbitEntity::Menu");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::Menu);
        },
    }
    run_trigger.set_target("camera_handler_cycle_state_camera", false);
    info!("post response: camera_handler_cycle_state_camera: {}", run_trigger.get("camera_handler_cycle_state_camera"));  
}

pub fn setup_3d_camera(
    mut commands: Commands,
) {
    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(0.0, 0.0, 0.0);
    camera.state.radius = 5.0;
    camera.state.pitch = 2.0f32.to_radians();
    camera.state.yaw = 0.0f32.to_radians();
    commands.spawn((
        camera,
        CameraWorld,
    ));
}

pub fn state_camera_orbit_entity_logic(
    camera_orbit_entity_state: ResMut<State<StateCameraOrbitEntity>>,
    mut camera_coord_tracker: ResMut<CameraHandler>,
    scene_meshes: Query<(Entity, &Name, &Transform)>,
    golf_balls: Query<(&GolfBall, &Transform)>,
    party: Res<Party>,
) {
    match camera_orbit_entity_state.get() {
        StateCameraOrbitEntity::Ball => {
            let active_player = party.active_player_get_player_id();
            for (golf_ball, transform) in golf_balls.iter() {
                if golf_ball.0.uuid == active_player {
                    camera_coord_tracker.current_coords = transform.translation;
                }
            }
        },    
        StateCameraOrbitEntity::Cup => {
            for (_entity, name, transform) in scene_meshes.iter() {
                if name.as_str() == "cup" {
                    camera_coord_tracker.current_coords = transform.translation;
                    break;
                }
            }        
        },
        StateCameraOrbitEntity::Menu => {
            for (_entity, name, transform) in scene_meshes.iter() {
                if name.as_str() == "cam_target" {
                    camera_coord_tracker.current_coords = transform.translation;
                    break;
                };
            }  
        },
        StateCameraOrbitEntity::FreePan => {},
    }
}

pub fn pan_orbit_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(&PanOrbitSettings, &mut StatePanOrbit, &mut Transform)>,
    camera_coord_tracker: Res<CameraHandler>,
    camera_orbit_entity_state: Res<State<StateCameraOrbitEntity>>,
) {
    // First, accumulate the total amount of mouse motion and scroll from all pending events:
    let mut total_motion: Vec2 = evr_motion.read().map(|ev| ev.delta).sum();
    total_motion.y = -total_motion.y; // Reverse Y

    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y -= ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y -= ev.y;
            }
        }
    }

    for (settings, mut state, mut transform) in &mut q_camera {
        // Determine the target based on the current camera state
        let target = match camera_orbit_entity_state.get() {
            StateCameraOrbitEntity::Ball | StateCameraOrbitEntity::Cup |
            StateCameraOrbitEntity::Menu => camera_coord_tracker.current_coords,
            StateCameraOrbitEntity::FreePan => state.center, // Use the original free pan center
        };

        let allow_interaction = match camera_orbit_entity_state.get() { // Disable all interactions in Menus
            StateCameraOrbitEntity::Menu => false,
            _ => true, // Enable interactions in all other states
        };

        // Accumulate values for pan, orbit, and zoom based on mouse input and key states
        let mut total_pan = Vec2::ZERO;
        let mut total_orbit = Vec2::ZERO;
        let mut total_zoom = Vec2::ZERO;

        // Only use manual panning if in FreePan mode and the appropriate key is pressed
        if let StateCameraOrbitEntity::FreePan = camera_orbit_entity_state.get() {
            if settings.pan_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
                total_pan -= total_motion * settings.pan_sensitivity;
            }
            if settings.scroll_action == Some(PanOrbitAction::Pan) {
                total_pan -= total_scroll_lines * settings.scroll_line_sensitivity * settings.pan_sensitivity;
                total_pan -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
            }
        }

        // Orbit logic - applicable in all modes
        if allow_interaction && settings.orbit_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_orbit -= total_motion * settings.orbit_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Orbit) {
            total_orbit -= total_scroll_lines * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
            total_orbit -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.orbit_sensitivity;
        }

        if allow_interaction && settings.zoom_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_zoom -= total_motion * settings.zoom_sensitivity;
        }
        if allow_interaction && settings.scroll_action == Some(PanOrbitAction::Zoom) {
            total_zoom -= total_scroll_lines * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
            total_zoom -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;
        }

        // Handle upside-down orbit reversal
        if settings.orbit_key.map(|key| kbd.just_pressed(key)).unwrap_or(false) {
            state.upside_down = state.pitch < -FRAC_PI_2 || state.pitch > FRAC_PI_2;
        }
        if state.upside_down {
            total_orbit.x = -total_orbit.x;
        }

        // Execute the actions based on accumulated values
        let mut any = false;

        // Apply zoom - multiply radius
        if total_zoom != Vec2::ZERO {
            any = true;
            state.radius *= (-total_zoom.y).exp();
        }

        // Apply orbit - modify yaw and pitch
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw += total_orbit.x;
            state.pitch -= total_orbit.y;

            // Wrap around yaw to stay within [-PI, PI]
            if state.yaw > PI {
                state.yaw -= TAU;
            }
            if state.yaw < -PI {
                state.yaw += TAU;
            }
            if state.pitch > PI {
                state.pitch += TAU;
            }
            if state.pitch < -PI {
                state.pitch -= TAU;
            }
        }

        // Apply pan - only in FreePan mode, or for following the target
        if let StateCameraOrbitEntity::FreePan = camera_orbit_entity_state.get() {
            if total_pan != Vec2::ZERO {
                any = true;
                let radius = state.radius;
                state.center += transform.right() * total_pan.x * radius;
                state.center += transform.up() * total_pan.y * radius;
            }
        } else {
            // In Ball or Cup mode, continuously update the target
            if target != state.center {
                any = true;
                state.center = target;
            }
        }

        // Update the camera's transform if anything changed
        if any || state.is_added() {
            transform.rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}