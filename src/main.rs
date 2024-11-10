use bevy::{prelude::*,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
};

// use bevy_editor_pls::prelude::*;

use minigolf::{Fonts, Interactable, OpIndex};
use minigolf::level_handler::level_handler::{gltf_handler_init, setup_ground, setup_light};
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
        .insert_state(LevelState::HoleTutorial)
        .insert_state(MapSetState::Tutorial)
        .insert_state(GameState::LoadingScreen)
        .insert_resource(Fonts::new())
        .insert_resource(OpIndex::new())
        .insert_resource(GLBPurgeID::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(GameStateHandler::new())
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, game_state_update.run_if(input_just_released(KeyCode::ArrowLeft)))
        .add_systems(Update, map_set_state_update.run_if(input_just_released(KeyCode::ArrowRight)))
        .add_systems(Update, level_state_update.run_if(input_just_released(KeyCode::ArrowUp)))
        // .add_systems(OnEnter(MapSetState::Tutorial), init_hole_n)
        .add_systems(OnEnter(LevelState::HoleTutorial), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole1), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole2), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole3), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole4), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole5), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole6), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole7), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole8), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole9), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole10), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole11), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole12), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole13), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole14), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole15), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole16), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole17), init_hole_n)
        .add_systems(OnEnter(LevelState::Hole18), init_hole_n)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole1), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole2), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole3), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole4), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole5), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole6), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole7), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole8), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole9), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole10), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole11), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole12), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole13), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole14), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole15), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole16), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole17), purge_glb_all)
        .add_systems(OnExit(LevelState::Hole18), purge_glb_all);
        app.run();
}

#[derive(Resource)]
struct GameStateHandler {
    current_level: i32,
    maps_index: i32,
}

impl GameStateHandler {
    fn new() -> Self {
        let current_level = 0;
        let maps_index = 0;
        GameStateHandler {
            current_level,
            maps_index,
        }
    }
}


#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    LoadingScreen,
    MenuMain,
    MenuSettings,
    MenuOnline,
    InGame,
    PostGameReview,
}

fn game_state_update(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match game_state.get() {
        GameState::LoadingScreen => {
            info!("GameState::MenuMain");
            next_game_state.set(GameState::MenuMain);
        },
        GameState::MenuMain => {
            info!("GameState::MenuSettings");
            next_game_state.set(GameState::MenuSettings);
        },
        GameState::MenuSettings => {
            info!("GameState::MenuOnline");
            next_game_state.set(GameState::MenuOnline);
        },
        GameState::MenuOnline => {
            info!("GameState::InGame");
            next_game_state.set(GameState::InGame);
        },
        GameState::InGame => {
            info!("GameState::PostGameReview");
            next_game_state.set(GameState::PostGameReview);
        },
        GameState::PostGameReview => {
            info!("GameState::LoadingScreen");
            next_game_state.set(GameState::LoadingScreen);
        },
        _ => {},
    }
}

fn game_state_logic(
    game_state: Res<State<GameState>>,
) {
    match game_state.get() {
        GameState::LoadingScreen => {},
        GameState::MenuMain => {},
        GameState::MenuSettings => {},
        GameState::MenuOnline => {},
        GameState::InGame => {},
        GameState::PostGameReview => {},
        _ => {},
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum MapSetState {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}

//will be UserInterface::select_a_hole_widget()
fn select_a_hole_widget() -> i32 {
    let target = 0;
    target
}

fn map_set_state_update(
    map_set_state: Res<State<MapSetState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match map_set_state.get() {
        MapSetState::Tutorial => {
            info!("MapSetState::Tutorial");
            gsh.current_level = 0;
            gsh.maps_index = 1;
            next_map_set_state.set(MapSetState::WholeCorse);
        },
        MapSetState::WholeCorse => {
            info!("MapSetState::WholeCorse");
            gsh.current_level = 0;
            gsh.maps_index = 18;
            next_map_set_state.set(MapSetState::FrontNine);
        },
        MapSetState::FrontNine => {
            info!("MapSetState::FrontNine");
            gsh.current_level = 0;
            gsh.maps_index = 9;
            next_map_set_state.set(MapSetState::BackNine);
        },
        MapSetState::BackNine => {
            info!("MapSetState::BackNine");
            gsh.current_level = 0;
            gsh.maps_index = 9;
            next_map_set_state.set(MapSetState::SelectAHole);
        },
        MapSetState::SelectAHole => {
            info!("MapSetState::SelectAHole");
            gsh.current_level = 0;
            gsh.maps_index = 1;
            let hole = select_a_hole_widget();
            match hole {
                0 => {},
                1 => {},
                2 => {},
                3 => {},
                4 => {},
                5 => {},
                6 => {},
                7 => {},
                8 => {},
                9 => {},
                10 => {},
                11 => {},
                12 => {},
                13 => {},
                14 => {},
                15 => {},
                16 => {},
                17 => {},
                18 => {},
                _ => {},
            }
            next_map_set_state.set(MapSetState::Tutorial);
        },
        _ => {},
    }
}

fn map_set_state_logic(
    map_set_state: Res<State<MapSetState>>,
) {
    match map_set_state.get() {
        MapSetState::Tutorial => {},
        MapSetState::WholeCorse => {},
        MapSetState::FrontNine => {},
        MapSetState::BackNine => {},
        MapSetState::SelectAHole => {},
        _ => {},
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum LevelState {
    #[default]
    HoleTutorial,
    Hole1,
    Hole2,
    Hole3,
    Hole4,
    Hole5,
    Hole6,
    Hole7,
    Hole8,
    Hole9,
    Hole10,
    Hole11,
    Hole12,
    Hole13,
    Hole14,
    Hole15,
    Hole16,
    Hole17,
    Hole18,
}

fn level_state_update(
    level_state: Res<State<LevelState>>,
    mut next_game_state: ResMut<NextState<LevelState>>,
    mut gsh: ResMut<GameStateHandler>,
) {
    match level_state.get() {
        LevelState::HoleTutorial => {
            gsh.current_level += 1;
            info!("LevelState::Hole1");
            next_game_state.set(LevelState::Hole1);
        },
        LevelState::Hole1 => {
            gsh.current_level += 1;
            info!("LevelState::Hole2");
            next_game_state.set(LevelState::Hole2);
        },
        LevelState::Hole2 => {
            gsh.current_level += 1;
            info!("LevelState::Hole3");
            next_game_state.set(LevelState::Hole3);
        },
        LevelState::Hole3 => {
            gsh.current_level += 1;
            info!("LevelState::Hole4");
            next_game_state.set(LevelState::Hole4);
        },
        LevelState::Hole4 => {
            gsh.current_level += 1;
            info!("LevelState::Hole5");
            next_game_state.set(LevelState::Hole5);
        },
        LevelState::Hole5 => {
            gsh.current_level += 1;
            info!("LevelState::Hole6");
            next_game_state.set(LevelState::Hole6);
        },
        LevelState::Hole6 => {
            gsh.current_level += 1;
            info!("LevelState::Hole7");
            next_game_state.set(LevelState::Hole7);
        },
        LevelState::Hole7 => {
            gsh.current_level += 1;
            info!("LevelState::Hole8");
            next_game_state.set(LevelState::Hole8);
        },
        LevelState::Hole8 => {
            gsh.current_level += 1;
            info!("LevelState::Hole9");
            next_game_state.set(LevelState::Hole9);
        },
        LevelState::Hole9 => {
            gsh.current_level += 1;
            info!("LevelState::Hole10");
            next_game_state.set(LevelState::Hole10);
        },
        LevelState::Hole10 => {
            gsh.current_level += 1;
            info!("LevelState::Hole11");
            next_game_state.set(LevelState::Hole11);
        },
        LevelState::Hole11 => {
            gsh.current_level += 1;
            info!("LevelState::Hole12");
            next_game_state.set(LevelState::Hole12);
        },
        LevelState::Hole12 => {
            gsh.current_level += 1;
            info!("LevelState::Hole13");
            next_game_state.set(LevelState::Hole13);
        },
        LevelState::Hole13 => {
            gsh.current_level += 1;
            info!("LevelState::Hole14");
            next_game_state.set(LevelState::Hole14);
        },
        LevelState::Hole14 => {
            gsh.current_level += 1;
            info!("LevelState::Hole15");
            next_game_state.set(LevelState::Hole15);
        },
        LevelState::Hole15 => {
            gsh.current_level += 1;
            info!("LevelState::Hole16");
            next_game_state.set(LevelState::Hole16);
        },
        LevelState::Hole16 => {
            gsh.current_level += 1;
            info!("LevelState::Hole17");
            next_game_state.set(LevelState::Hole17);
        },
        LevelState::Hole17 => {
            gsh.current_level += 1;
            info!("LevelState::Hole18");
            next_game_state.set(LevelState::Hole18);
        },
        LevelState::Hole18 => {
            gsh.current_level = 0;
            info!("LevelState::HoleTutorial");
            next_game_state.set(LevelState::HoleTutorial);
        },
        _ => {},
    }
}

fn level_state_logic(
    level_state: Res<State<LevelState>>,
) {
    match level_state.get() {
        LevelState::HoleTutorial => {},
        LevelState::Hole1 => {},
        LevelState::Hole2 => {},
        LevelState::Hole3 => {},
        LevelState::Hole4 => {},
        LevelState::Hole5 => {},
        LevelState::Hole6 => {},
        LevelState::Hole7 => {},
        LevelState::Hole8 => {},
        LevelState::Hole9 => {},
        LevelState::Hole10 => {},
        LevelState::Hole11 => {},
        LevelState::Hole12 => {},
        LevelState::Hole13 => {},
        LevelState::Hole14 => {},
        LevelState::Hole15 => {},
        LevelState::Hole16 => {},
        LevelState::Hole17 => {},
        LevelState::Hole18 => {},
        _ => {},
    }
}

// When entering state 

fn init_hole_n(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
    gsh: Res<GameStateHandler>,
) {
    info!("init_hole_n: Init Hole {}", gsh.current_level);
    gltf_handler_init_hole_n(asset_server, commands, op_index, glb_storage, gsh.current_level);
}

#[derive(Clone, Resource)]
struct GLBStorageID {
    glb: Vec<String>,
}

impl GLBStorageID {
    fn new() -> Self {
        let mut glb: Vec<String> = Vec::new();
        let map_t = "glb/boilerplate_level_tutorial.glb".to_string();
        glb.push(map_t);
        let map_1: String = String::from("glb/boilerplate_level_1.glb");
        let map_2: String = String::from("glb/boilerplate_level_2.glb");
        let map_3: String = String::from("glb/boilerplate_level_3.glb");
        let map_4: String = String::from("glb/boilerplate_level_4.glb");
        let map_5: String = String::from("glb/boilerplate_level_5.glb");
        let map_6: String = String::from("glb/boilerplate_level_6.glb");
        let map_7: String = String::from("glb/boilerplate_level_7.glb");
        let map_8: String = String::from("glb/boilerplate_level_8.glb");
        let map_9: String = String::from("glb/boilerplate_level_9.glb");
        let map_10: String = String::from("glb/boilerplate_level_10.glb");
        let map_11: String = String::from("glb/boilerplate_level_11.glb");
        let map_12: String = String::from("glb/boilerplate_level_12.glb");
        let map_13: String = String::from("glb/boilerplate_level_13.glb");
        let map_14: String = String::from("glb/boilerplate_level_14.glb");
        let map_15: String = String::from("glb/boilerplate_level_15.glb");
        let map_16: String = String::from("glb/boilerplate_level_16.glb");
        let map_17: String = String::from("glb/boilerplate_level_17.glb");
        let map_18: String = String::from("glb/boilerplate_level_18.glb");
        glb.push(map_1);
        glb.push(map_2);
        glb.push(map_3);
        glb.push(map_4);
        glb.push(map_5);
        glb.push(map_6);
        glb.push(map_7);
        glb.push(map_8);
        glb.push(map_9);
        glb.push(map_10);
        glb.push(map_11);
        glb.push(map_12);
        glb.push(map_13);
        glb.push(map_14);
        glb.push(map_15);
        glb.push(map_16);
        glb.push(map_17);
        glb.push(map_18);
        info!("{:?}", &glb);
        GLBStorageID {
            glb,
        }
    }
}

fn gltf_handler_init_hole_n(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut op_index: ResMut<OpIndex>,
    glb_storage: Res<GLBStorageID>,
    hole: i32,
) {
    if let Some(glb_file) = glb_storage.glb.get((hole) as usize) {
        let info_dump = glb_file.clone();

        commands.spawn(SceneBundle {
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset(info_dump)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Interactable); 
        op_index.add_ui_entity();    
    } else {
        warn!("Target map was not valid. Hole was out of bounds, 0 for the tutorial, 1-18 for the golf holes.");
    };
}

// When exiting state 

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

fn purge_glb_all_prep(
    purge: &mut ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    let targets = glb_storage.clone();
    for asset_to_despawn in targets.glb.iter() {
        purge.glb.push(asset_to_despawn.to_string());
    }
}

fn purge_glb_all(
    mut commands: Commands,
    scene_query: Query<(Entity, &Handle<Scene>)>,
    asset_server: Res<AssetServer>,
    mut purge: ResMut<GLBPurgeID>,
    glb_storage: Res<GLBStorageID>,
) {
    purge_glb_all_prep(&mut purge, glb_storage);
    gltf_handler_purge(commands, scene_query, asset_server, purge);
}