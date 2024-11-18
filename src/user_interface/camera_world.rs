use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
    use bevy::prelude::*;
    
    use std::f32::consts::{FRAC_PI_2, PI, TAU};
    
    use bevy_mod_raycast::prelude::*;
    
    use crate::{
        CameraCoordTracker, 
        CameraOrbitEntityState, 
        CameraWorld, 
        Ground, 
        PanOrbitAction,
        PanOrbitCameraBundle,
        PanOrbitSettings,
        PanOrbitState,
        RigidBody,
    };
    
    pub fn setup_3d_camera(
        mut commands: Commands,
    ) {
        let mut camera = PanOrbitCameraBundle::default();
        // Position our camera using our component,
        // not Transform (it would get overwritten)
        camera.state.center = Vec3::new(0.0, 0.0, 0.0);
        camera.state.radius = 38.0;
        camera.state.pitch = -12.0f32.to_radians();
        camera.state.yaw = -17.0f32.to_radians();
        commands.spawn((
            camera,
            CameraWorld,
        ));
    }
    
    
    
    pub fn camera_orbit_entity_state_logic(
        mut camera_orbit_entity_state: ResMut<State<CameraOrbitEntityState>>,
        mut camera_coord_tracker: ResMut<CameraCoordTracker>,
        scene_meshes: Query<(Entity, &Name, &Transform)>,
        mut q_rigid_body: Query<(&RigidBody, &Transform)>,
    ) {
        let mut ball_rigid_body_coords: Vec3 = Vec3::new(0.0, 0.0, 0.0); 
        for (_, transform) in q_rigid_body.iter() {
            ball_rigid_body_coords = transform.translation.clone();
        }
        match camera_orbit_entity_state.get() {
            CameraOrbitEntityState::Ball => {
                for (entity, name, transform) in scene_meshes.iter() {
                    if name.as_str() == "ball" {
                        camera_coord_tracker.current_coords = ball_rigid_body_coords;
                        break;
                    }
                }        
            }
            CameraOrbitEntityState::Cup => {
                for (entity, name, transform) in scene_meshes.iter() {
                    if name.as_str() == "cup" {
                        camera_coord_tracker.current_coords = transform.translation;
                        break;
                    }
                }        
            }
            CameraOrbitEntityState::GameInit | CameraOrbitEntityState::LeaderBoard |
            CameraOrbitEntityState::MenuLocal | CameraOrbitEntityState::MenuOnline |
            CameraOrbitEntityState::MainMenu | CameraOrbitEntityState::MenuPreferences => {
                for (entity, name, transform) in scene_meshes.iter() {
                    if name.as_str() == "cam_target" {
                        camera_coord_tracker.current_coords = transform.translation;
                        break;
                    }
                }  
            }      
            CameraOrbitEntityState::FreePan => {    
            }
        }
    }
    
    pub fn camera_orbit_entity_state_update(    
        camera_orbit_entity_state: Res<State<CameraOrbitEntityState>>,
        mut next_camera_orbit_entity_state: ResMut<NextState<CameraOrbitEntityState>>,
    ) {
        match camera_orbit_entity_state.get() {
            CameraOrbitEntityState::FreePan => {
                info!("CameraOrbitEntityState::LeaderBoard");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::MainMenu);
            },
            CameraOrbitEntityState::LeaderBoard => {
                info!("CameraOrbitEntityState::GameInit");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::GameInit);
            },
            CameraOrbitEntityState::GameInit => {
                info!("CameraOrbitEntityState::MenuLocal");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::MenuLocal);
            },
            CameraOrbitEntityState::MenuLocal => {
                info!("CameraOrbitEntityState::MenuOnline");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::MenuOnline);
            },
            CameraOrbitEntityState::MenuOnline => {
                info!("CameraOrbitEntityState::MainMenu");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::MainMenu);
            },
            CameraOrbitEntityState::MainMenu => {
                info!("CameraOrbitEntityState::MenuPreferences");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::MenuPreferences);
            },
            CameraOrbitEntityState::MenuPreferences => {
                info!("CameraOrbitEntityState::Ball");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::Cup);
            },
            CameraOrbitEntityState::Ball => {
                info!("CameraOrbitEntityState::Cup");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::Cup);
            },
            CameraOrbitEntityState::Cup => {
                info!("CameraOrbitEntityState::FreePan");
                next_camera_orbit_entity_state.set(CameraOrbitEntityState::FreePan);
            },
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
                CameraOrbitEntityState::MainMenu | CameraOrbitEntityState::MenuPreferences | 
                CameraOrbitEntityState::MenuLocal | CameraOrbitEntityState::MenuOnline | 
                CameraOrbitEntityState::LeaderBoard | CameraOrbitEntityState::GameInit |
                CameraOrbitEntityState::Ball | CameraOrbitEntityState::Cup => {
                    camera_coord_tracker.current_coords
                }
                CameraOrbitEntityState::FreePan => state.center, // Use the original free pan center
            };
    
            let allow_interaction = match camera_orbit_entity_state.get() { // Disable all interactions in MainMenu * LeaderBoard
                CameraOrbitEntityState::MainMenu => false,
                CameraOrbitEntityState::MenuLocal => false,
                // CameraOrbitEntityState::LeaderBoard => false,
                _ => true, // Enable interactions in all other states
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
            if allow_interaction && settings.orbit_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
                total_orbit -= total_motion * settings.orbit_sensitivity;
            }
            if settings.scroll_action == Some(PanOrbitAction::Orbit) {
                total_orbit -= total_scroll_lines * settings.scroll_line_sensitivity * settings.orbit_sensitivity;
                total_orbit -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.orbit_sensitivity;
            }
    
            // Zoom logic - applicable in all modes
            if allow_interaction && settings.zoom_key.map(|key| kbd.pressed(key)).unwrap_or(false) {
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