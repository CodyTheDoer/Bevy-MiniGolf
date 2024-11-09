use bevy::{prelude::*,
    // ecs::world::World,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
    // tasks::IoTaskPool, 
    // utils::Duration,
};

// use std::{fs::File, io::Write};

// use bevy_editor_pls::prelude::*;

use minigolf::{Fonts, Interactable, OpIndex};
use minigolf::level_handler::level_handler::{gltf_handler_init, setup_ground, setup_light}; //query_and_despawn_scene, query_and_update_scene};
use minigolf::user_interface::camera_world::setup_3d_camera;
use minigolf::user_interface::user_interface::{fire_ray, release_ray, draw_cursor, setup_ui};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Calculator Simulator".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    resizable: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..Default::default()
                    },
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false, // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        // .add_plugins(EditorPlugin::default())
        .insert_state(AppState::Game)
        .insert_resource(Fonts::new())
        .insert_resource(OpIndex::new())
        .insert_resource(GLBPurgeID::new())
        .insert_resource(GLBStorageID::new())
        // .add_systems(Startup, gltf_handler_init)
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        // .add_systems(Update, query_and_despawn_scene.run_if(input_pressed(MouseButton::Right)))
        // .add_systems(Update, query_and_update_scene.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, app_state_logic)
        .add_systems(Update, app_state_cycle.run_if(input_just_released(KeyCode::ArrowUp)))
        .add_systems(OnEnter(AppState::Game), app_state_game_logic_enter)
        .add_systems(OnExit(AppState::Game), app_state_game_logic_exit)
        .add_systems(OnEnter(AppState::Menu), app_state_menu_logic_enter)
        .add_systems(OnExit(AppState::Menu), app_state_menu_logic_exit)
        .add_systems(OnEnter(AppState::Paused), app_state_paused_logic_enter)
        .add_systems(OnExit(AppState::Paused), app_state_paused_logic_exit)
        .add_systems(OnEnter(AppState::ClearGLB), app_state_clear_glb_enter);
        app.run();
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Game,
    Menu,
    Paused,
    ClearGLB,
}

fn app_state_cycle(
    app_state: Res<State<AppState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            next_game_state.set(AppState::Menu);
        },
        AppState::Menu => {
            next_game_state.set(AppState::Paused);
        },
        AppState::Paused => {
            next_game_state.set(AppState::ClearGLB);
        },
        AppState::ClearGLB => {
            next_game_state.set(AppState::Game);
        },
        _ => {},
    }
}

fn app_state_logic(
    app_state: Res<State<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            // info!("AppState::Game");
        },
        AppState::Menu => {
            // info!("AppState::Menu");
        },
        AppState::Paused => {
            // info!("AppState::Paused");
        },
        AppState::ClearGLB => {
            // info!("AppState::Paused");
        },
        _ => {},
    }
}

fn app_state_game_logic_enter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
) {
    info!("AppState::Game::OnEnter - Init Hole 1");
    gltf_handler_init_hole_n(asset_server, commands, op_index, glb_storage, 1);
}

fn app_state_game_logic_exit() {
    info!("AppState::Game::OnExit");
}

fn app_state_menu_logic_enter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
) {
    info!("AppState::Menu::OnEnter - Init Hole 2");
    gltf_handler_init_hole_n(asset_server, commands, op_index, glb_storage, 2);
}

fn app_state_menu_logic_exit(
) {
    info!("AppState::Menu::OnExit");
}

fn app_state_paused_logic_enter(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
) {
    info!("AppState::Paused::OnEnter - Init Hole 3");
    gltf_handler_init_hole_n(asset_server, commands, op_index, glb_storage, 3);
}

fn app_state_paused_logic_exit(
) {
    info!("AppState::Paused::OnExit");
}

fn app_state_clear_glb_enter(
    mut commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    mut purge: ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    purge_add_all(&mut purge, glb_storage);
    gltf_handler_purge(commands, scene_query, asset_server, purge);
}

#[derive(Clone, Resource)]
struct GLBStorageID {
    glb: Vec<String>,
}

impl GLBStorageID {
    fn new() -> Self {
        let mut glb: Vec<String> = Vec::new();
        let map_1: String = String::from("cube_blue.glb");
        let map_2: String = String::from("cube_terracotta.glb");
        let map_3: String = String::from("cube_toxic.glb");
        glb.push(map_1);
        glb.push(map_2);
        glb.push(map_3);
        GLBStorageID {
            glb,
        }
    }
}

#[derive(Clone, Resource)]
struct GLBPurgeID {
    glb: Vec<String>,
}

impl GLBPurgeID {
    fn new() -> Self {
        let glb: Vec<String> = Vec::new();
        GLBPurgeID {
            glb,
        }
    }
}

fn remove_match_from_vec(vec: &mut ResMut<GLBPurgeID>, pattern: &str) {
    if let Some(pos) = vec.glb.iter().position(|x| x == pattern) {
        vec.glb.remove(pos);
    }
}

// When exit state 
fn gltf_handler_purge(
    mut commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    mut purge: ResMut<GLBPurgeID>,
) {
    let targets = purge.clone();
    for asset_to_despawn in targets.glb.iter() {
        let target_asset = format!("{}#Scene0",asset_to_despawn);
        // We load the specific scene handle to compare it directly
        let despawn_target: Handle<Scene> = asset_server.load(target_asset);// format!("{}#Scene0", glb_file)
        for (entity, scene_handle) in scene_query.iter() {
            // Check if the scene handle matches the target handle
            if scene_handle.id() == despawn_target.id() {
                commands.entity(entity).despawn_recursive();
                info!("Despawned Entity: {:?}", entity);
            }
        }
        remove_match_from_vec(&mut purge, asset_to_despawn);
    }
}

fn purge_add_all(
    purge: &mut ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    let targets = glb_storage.clone();
    for asset_to_despawn in targets.glb.iter() {
        purge.glb.push(asset_to_despawn.to_string());
    }
}

fn gltf_handler_init_hole_n(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
    hole: i32,
) {
    if let Some(glb_file) = glb_storage.glb.get((hole - 1) as usize) {
        let info_dump = glb_file.clone();

        commands.spawn(SceneBundle {
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset(info_dump)),
            transform: Transform::from_xyz(-2.0, 0.0, 5.0),
            ..default()
        })
        .insert(Interactable); 
        op_index.add_ui_entity();    
    } else {
        warn!("Target map was not valid. Hole was out of bounds, 0 for the tutorial, 1-18 for the golf holes.");
    };

}
