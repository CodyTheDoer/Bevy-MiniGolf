use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy_mod_raycast::prelude::*;

use crate::{CameraCoordTracker, CameraOrbitEntityState, CameraWorld, Ground};

pub fn setup_3d_camera(
    mut commands: Commands,
) {
    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(0.0, 0.0, 0.0);
    camera.state.radius = 20.0;
    camera.state.pitch = -55.0f32.to_radians();
    camera.state.yaw = 0.0f32.to_radians();
    commands.spawn((
        camera,
        CameraWorld,
    ));
}

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

// The internal state of the pan-orbit controller
#[derive(Component, Debug)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// Key to hold for panning
    pub pan_key: Option<KeyCode>,
    /// Key to hold for orbiting
    pub orbit_key: Option<KeyCode>,
    /// Key to hold for zooming
    pub zoom_key: Option<KeyCode>,
    /// What action is bound to the scroll wheel?
    pub scroll_action: Option<PanOrbitAction>,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
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

pub fn camera_orbit_entity_state_logic(
    mut camera_orbit_entity_state: ResMut<State<CameraOrbitEntityState>>,
    mut camera_coord_tracker: ResMut<CameraCoordTracker>,
    scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    match camera_orbit_entity_state.get() {
        CameraOrbitEntityState::Ball => {}
        CameraOrbitEntityState::Cup => {}
        CameraOrbitEntityState::FreePan => {}
    }
}

pub fn pan_orbit_camera(
    kbd: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(&PanOrbitSettings, &mut PanOrbitState, &mut Transform)>,
    camera_coord_tracker: Res<CameraCoordTracker>,
    camera_orbit_entity_state: Res<State<CameraOrbitEntityState>>,
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
            CameraOrbitEntityState::Ball | CameraOrbitEntityState::Cup => {
                camera_coord_tracker.current_coords
            }
            CameraOrbitEntityState::FreePan => state.center, // Use the original free pan center
        };

        // Accumulate values for pan, orbit, and zoom based on mouse input and key states
        let mut total_pan = Vec2::ZERO;
        let mut total_orbit = Vec2::ZERO;
        let mut total_zoom = Vec2::ZERO;

        // Only use manual panning if in FreePan mode and the appropriate key is pressed
        if let CameraOrbitEntityState::FreePan = camera_orbit_entity_state.get() {
            if settings.pan_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
                total_pan -= total_motion * settings.pan_sensitivity;
            }
            if settings.scroll_action == Some(PanOrbitAction::Pan) {
                total_pan -= total_scroll_lines * settings.scroll_line_sensitivity * settings.pan_sensitivity;
                total_pan -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.pan_sensitivity;
            }
        }

        // Orbit logic - applicable in all modes
        if settings.orbit_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_orbit -= total_motion * settings.orbit_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Orbit) {
            total_orbit -= total_scroll_lines * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
            total_orbit -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.orbit_sensitivity;
        }

        // Zoom logic - applicable in all modes
        if settings.zoom_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
            total_zoom -= total_motion * settings.zoom_sensitivity;
        }
        if settings.scroll_action == Some(PanOrbitAction::Zoom) {
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
        if let CameraOrbitEntityState::FreePan = camera_orbit_entity_state.get() {
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
