// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    time::common_conditions::on_timer, 
    utils::Duration,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
// use bevy_editor_pls::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_rapier3d::prelude::*;

// --- States --- //
use minigolf::{ 
    ArrowState,
    CameraOrbitEntityState,
    GameState,
    LeaderBoardState,
    LevelState,
    MapSetState,
    MenuState,
    PartyConnectionState,
    PlayerCompletionState,
    PlayThroughStyleState,
    TurnState,
};

// --- Resources --- //
use minigolf::{
    BonkHandler,
    CameraCoordTracker,
    Fonts, 
    GameHandler, 
    GLBStorageID, 
    LevelHandler,
    PanOrbitState,
    PanOrbitSettings,
    Party,
    Player,
};

// --- enums --- //
use minigolf::{
    StateUpdateRef,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    camera_orbit_entity_state_logic,
    camera_orbit_entity_state_update,
};
use minigolf::user_interface::user_interface::{
    bonk_gizmo,
    ray_fire, 
    ray_release, 
    draw_cursor,
    setup_ui,
    update_ui,
};

// --- Level Handler Import --- //
use minigolf::level_handler::level_handler::{
    init_level_glb, 
    setup_ground, 
    setup_light, 
    purge_glb,
    purge_rigid_bodies,
};

// --- Physics Handler Import --- //
use minigolf::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
    bonk_step_start,
    bonk_step_mid,
    bonk_step_end,
    collision_events_listener,
    performance_physics_setup,
};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minigolf".into(),
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

        // --- Additional Plugins --- //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())

        // --- State Initialization --- //
        .insert_state(ArrowState::Idle)
        .insert_state(CameraOrbitEntityState::MainMenu)
        .insert_state(GameState::LoadingScreen)
        .insert_state(LeaderBoardState::Mixed)
        .insert_state(LevelState::MainMenu)
        .insert_state(MapSetState::Tutorial)
        .insert_state(MenuState::NoSelection)
        .insert_state(PartyConnectionState::Local)
        .insert_state(PlayThroughStyleState::Proximity)
        .insert_state(TurnState::Idle)

        // --- Resource Initialization --- //
        .insert_resource(BonkHandler::new())
        .insert_resource(CameraCoordTracker::new())
        .insert_resource(Fonts::new())
        .insert_resource(GameHandler::new())
        .insert_resource(GLBStorageID::new())
        .insert_resource(LevelHandler::new())
        .insert_resource(Party::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, performance_physics_setup)

        // User Interface //
        .add_systems(Update, update_ui) // Driving HUD Features with State info
        .add_systems(Update, draw_cursor)
        .add_systems(Update, ray_fire.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, ray_release.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, bonk_gizmo.run_if(in_state(ArrowState::DrawingArrow)))
        
        // Camera //
        .add_systems(Update, camera_orbit_entity_state_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, camera_orbit_entity_state_update.run_if(input_just_released(KeyCode::KeyC)))

        // Physics //
        // .add_systems(Update, add_physics_query_and_update_scene.run_if(input_just_released(MouseButton::Right)))
        .add_systems(Update, collision_events_listener)
        .add_systems(Update, bonk_step_start.run_if(input_just_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_mid.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, bonk_step_end.run_if(input_just_released(MouseButton::Right)))
        .add_systems(Update, add_physics_query_and_update_scene.run_if(asset_event_listener))

        // --- OnEnter State Reaction Level Initialization --- //        
        .add_systems(OnEnter(LevelState::MainMenu), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole1), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole2), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole3), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole4), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole5), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole6), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole7), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole8), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole9), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole10), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole11), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole12), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole13), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole14), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole15), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole16), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole17), init_level_glb)
        .add_systems(OnEnter(LevelState::Hole18), init_level_glb)
        .add_systems(OnEnter(LevelState::HoleTutorial), init_level_glb)
        .add_systems(OnEnter(LevelState::MenuLeaderBoard), init_level_glb)
        .add_systems(OnEnter(LevelState::MenuLocal), init_level_glb)
        .add_systems(OnEnter(LevelState::MenuOnline), init_level_glb)
        .add_systems(OnEnter(LevelState::MenuPreferences), init_level_glb)
        .add_systems(OnEnter(LevelState::MenuPlayer), init_level_glb)

        // --- OnExit State Reaction Level Purge --- //
        .add_systems(OnExit(LevelState::MainMenu), purge_glb)
        .add_systems(OnExit(LevelState::Hole1), purge_glb)
        .add_systems(OnExit(LevelState::Hole2), purge_glb)
        .add_systems(OnExit(LevelState::Hole3), purge_glb)
        .add_systems(OnExit(LevelState::Hole4), purge_glb)
        .add_systems(OnExit(LevelState::Hole5), purge_glb)
        .add_systems(OnExit(LevelState::Hole6), purge_glb)
        .add_systems(OnExit(LevelState::Hole7), purge_glb)
        .add_systems(OnExit(LevelState::Hole8), purge_glb)
        .add_systems(OnExit(LevelState::Hole9), purge_glb)
        .add_systems(OnExit(LevelState::Hole10), purge_glb)
        .add_systems(OnExit(LevelState::Hole11), purge_glb)
        .add_systems(OnExit(LevelState::Hole12), purge_glb)
        .add_systems(OnExit(LevelState::Hole13), purge_glb)
        .add_systems(OnExit(LevelState::Hole14), purge_glb)
        .add_systems(OnExit(LevelState::Hole15), purge_glb)
        .add_systems(OnExit(LevelState::Hole16), purge_glb)
        .add_systems(OnExit(LevelState::Hole17), purge_glb)
        .add_systems(OnExit(LevelState::Hole18), purge_glb)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb)
        .add_systems(OnExit(LevelState::MenuLeaderBoard), purge_glb)
        .add_systems(OnExit(LevelState::MenuLocal), purge_glb)
        .add_systems(OnExit(LevelState::MenuOnline), purge_glb)
        .add_systems(OnExit(LevelState::MenuPreferences), purge_glb)
        .add_systems(OnExit(LevelState::MenuPlayer), purge_glb)
        
        .add_systems(OnExit(LevelState::Hole1), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole2), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole3), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole4), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole5), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole6), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole7), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole8), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole9), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole10), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole11), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole12), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole13), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole14), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole15), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole16), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole17), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::Hole18), purge_rigid_bodies)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_rigid_bodies)
        
        // --- OnEnter State Reaction Game State --- //
        .add_systems(OnEnter(GameState::Menus), game_state_response_menus)
        
        // --- OnEnter State Reaction Menu State --- //
        .add_systems(OnEnter(MenuState::Local), menu_state_response_local)
        .add_systems(OnEnter(MenuState::Online), menu_state_response_online)
        .add_systems(OnEnter(MenuState::LeaderBoard), menu_state_response_leader_board)
        .add_systems(OnEnter(MenuState::Player), menu_state_response_player)
        .add_systems(OnEnter(MenuState::Preferences), menu_state_response_preferences)
        .add_systems(OnEnter(MenuState::Tutorial), menu_state_response_tutorial)
        
        // --- OnEnter State Reaction Turn State --- //
        .add_systems(OnEnter(TurnState::HoleComplete), turn_state_response_hole_complete)
        .add_systems(OnEnter(TurnState::TurnReset), turn_state_response_turn_reset);



    // --- Network Integration --- //
    app.add_systems(Startup, start_socket)
        .add_systems(Update, receive_messages)
        .add_systems(Update, remote_state_change_monitor)
        .add_systems(Update, send_message.run_if(on_timer(Duration::from_secs(5))))
        .add_event::<OnlineStateChange>();


    app.run();
}

fn asset_event_listener(
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    // mut assets: ResMut<Assets<Mesh>>,
) -> bool {
    let mut event_occurred = false;
    for event in ev_asset.read() {
        event_occurred = true;
    };
    event_occurred
}

// --- OnEnter: Game State --- //

pub fn game_state_response_menus(
    mut party: ResMut<Party>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
){
    next_menu_state.set(MenuState::NoSelection);
    next_turn.set(TurnState::Idle);
    next_leader_board_state.set(LeaderBoardState::Mixed);
    next_level.set(LevelState::MainMenu);
}

// --- OnEnter: Menu State --- //

fn menu_state_response_local(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_local();
    next_game_state.set(GameState::GameInitLocal);
    next_level_state.set(LevelState::MenuLocal);    
    next_camera_state.set(CameraOrbitEntityState::MenuLocal);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = 17.0f32.to_radians();
    }
}

fn menu_state_response_online(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_online();
    next_game_state.set(GameState::GameInitOnline);
    next_level_state.set(LevelState::MenuOnline);    
    next_camera_state.set(CameraOrbitEntityState::GameInit);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = 17.0f32.to_radians();
    }
}

fn menu_state_response_leader_board(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_leader_board();
    next_menu_state.set(MenuState::LeaderBoard);
    next_game_state.set(GameState::LeaderBoard);
    next_level_state.set(LevelState::MenuLeaderBoard);
    next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -7.0f32.to_radians();
    }
}

fn menu_state_response_tutorial(
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_map_set_state: ResMut<NextState<MapSetState>>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    party.start_game();
    game_handler.init_tutorial();
    next_level_state.set(LevelState::HoleTutorial);
    next_menu_state.set(MenuState::NoSelection);
    next_leader_board_state.set(LeaderBoardState::InGame);
    next_map_set_state.set(MapSetState::Tutorial);
    next_game_state.set(GameState::InGame);
    next_turn_state.set(TurnState::Turn);
    next_camera_state.set(CameraOrbitEntityState::Ball);
    for mut state in pan_orbit_camera_query.iter_mut() {
        state.radius = 2.0;
        state.pitch = -8.0f32.to_radians();
        state.yaw = 22.0f32.to_radians();
    }
}

fn menu_state_response_player(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_player();
    next_level_state.set(LevelState::MenuPlayer);
    next_menu_state.set(MenuState::Player);
    next_game_state.set(GameState::MenuPlayer);
    next_camera_state.set(CameraOrbitEntityState::MenuPlayer);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -10.0f32.to_radians();
    }
}

fn menu_state_response_preferences(
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    game_handler.init_menu_preferences();
    next_menu_state.set(MenuState::Preferences);
    next_game_state.set(GameState::Preferences);
    next_level_state.set(LevelState::MenuPreferences);
    next_camera_state.set(CameraOrbitEntityState::MenuPreferences);
    for mut state in pan_orbit_camera_query.iter_mut() {
        info!("{:?}", state);
        state.radius = 38.0;
        state.pitch = -12.0f32.to_radians();
        state.yaw = -12.0f32.to_radians();
    }
}

// --- OnEnter: Turn State --- //

fn turn_state_response_hole_complete(
    mut party: ResMut<Party>,
    map_set_state: Res<State<MapSetState>>,
    level: ResMut<State<LevelState>>,
    level_handler: Res<LevelHandler>,
    mut game_handler: ResMut<GameHandler>,
    mut next_leader_board_state: ResMut<NextState<LeaderBoardState>>,
    mut next_camera_state: ResMut<NextState<CameraOrbitEntityState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut next_turn: ResMut<NextState<TurnState>>,
    mut pan_orbit_camera_query: Query<&mut PanOrbitState>,
) {
    party.active_player_finished_hole(); // Reads active player index and updates target Player's state
    
    let current_level = party.get_active_level();
    let party_size = party.get_party_size();
    
    if party_size == 1 {
        let maps = match map_set_state.get() {
            MapSetState::Tutorial => {
                party.end_game(); // Sets players to NotInGame
                next_game_state.set(GameState::PostGameReview);
                next_turn.set(TurnState::Idle);
                game_handler.init_postgame_leaderboard(party); // Set's target for level handling
                next_leader_board_state.set(LeaderBoardState::PostGame);
                next_level.set(LevelState::MenuLeaderBoard);
                next_camera_state.set(CameraOrbitEntityState::LeaderBoard);
                for mut state in pan_orbit_camera_query.iter_mut() {
                    info!("{:?}", state);
                    state.radius = 38.0;
                    state.pitch = -12.0f32.to_radians();
                    state.yaw = -17.0f32.to_radians();
                }
            },
            MapSetState::WholeCorse => {
                if current_level == 18 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::FrontNine => {
                if current_level == 9 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::BackNine => {
                if current_level == 18 {
                    todo!(); // End Game Leaderboard
                } else {
                    let set_next_level = level_handler.next_level(current_level);
                    next_level.set(set_next_level);
                    game_handler.next_level();
                    party.next_level();
                    next_turn.set(TurnState::Turn);
                }
            },
            MapSetState::SelectAHole => {
                    todo!(); // End Game Leaderboard
            },
        };
    }
    
    // next_turn_state.set(TurnState::)
}

fn turn_state_response_turn_reset(
    mut party: ResMut<Party>,
    mut next_turn_state: ResMut<NextState<TurnState>>,
) {
}
// fn turn_state_response_new_game() {}
// fn turn_state_response_next_turn() {}
// fn turn_state_response_game_complete() {}












// Matchbox Network integration

fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

fn send_message(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "Hello";
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

#[derive(Debug, Event)]
struct OnlineStateChange;

fn server_parse_message(
    message: &str,
    game_handler: &mut ResMut<GameHandler>,
    online_event_handler: &mut EventWriter<OnlineStateChange>,
) {
    let mut split: &Vec<&str> = &message.split("::").collect();
    if split.len() != 2 {
        info!("Invalid message format");
        return;
    }

    let parsed_state = match split[0] {
        "GameState" => match split[1] {
            "GameInitLocal" => Some(StateUpdateRef::GameState(GameState::GameInitLocal)),
            "GameInitOnline" => Some(StateUpdateRef::GameState(GameState::GameInitOnline)),
            "LeaderBoard" => Some(StateUpdateRef::GameState(GameState::LeaderBoard)),
            "LoadingScreen" => Some(StateUpdateRef::GameState(GameState::LoadingScreen)),
            "InGame" => Some(StateUpdateRef::GameState(GameState::InGame)),
            "InGamePaused" => Some(StateUpdateRef::GameState(GameState::InGamePaused)),
            "Menus" => Some(StateUpdateRef::GameState(GameState::Menus)),
            "PostGameReview" => Some(StateUpdateRef::GameState(GameState::PostGameReview)),
            "Preferences" => Some(StateUpdateRef::GameState(GameState::Preferences)),
            "MenuPlayer" => Some(StateUpdateRef::GameState(GameState::MenuPlayer)),
            _ => None,
        },
        "LevelState" => match split[1] {
            "MainMenu" => Some(StateUpdateRef::LevelState(LevelState::MainMenu)),
            "Hole1" => Some(StateUpdateRef::LevelState(LevelState::Hole1)),
            "Hole2" => Some(StateUpdateRef::LevelState(LevelState::Hole2)),
            "Hole3" => Some(StateUpdateRef::LevelState(LevelState::Hole3)),
            "Hole4" => Some(StateUpdateRef::LevelState(LevelState::Hole4)),
            "Hole5" => Some(StateUpdateRef::LevelState(LevelState::Hole5)),
            "Hole6" => Some(StateUpdateRef::LevelState(LevelState::Hole6)),
            "Hole7" => Some(StateUpdateRef::LevelState(LevelState::Hole7)),
            "Hole8" => Some(StateUpdateRef::LevelState(LevelState::Hole8)),
            "Hole9" => Some(StateUpdateRef::LevelState(LevelState::Hole9)),
            "Hole10" => Some(StateUpdateRef::LevelState(LevelState::Hole10)),
            "Hole11" => Some(StateUpdateRef::LevelState(LevelState::Hole11)),
            "Hole12" => Some(StateUpdateRef::LevelState(LevelState::Hole12)),
            "Hole13" => Some(StateUpdateRef::LevelState(LevelState::Hole13)),
            "Hole14" => Some(StateUpdateRef::LevelState(LevelState::Hole14)),
            "Hole15" => Some(StateUpdateRef::LevelState(LevelState::Hole15)),
            "Hole16" => Some(StateUpdateRef::LevelState(LevelState::Hole16)),
            "Hole17" => Some(StateUpdateRef::LevelState(LevelState::Hole17)),
            "Hole18" => Some(StateUpdateRef::LevelState(LevelState::Hole18)),
            "HoleTutorial" => Some(StateUpdateRef::LevelState(LevelState::HoleTutorial)),
            _ => None,
        },
        _ => None,
    };
    
    if let Some(state) = parsed_state {
        info!("Updated state valid: {:?}", state);
        game_handler.auth_server_update_received(Some(state));
        online_event_handler.send(OnlineStateChange);
    } else {
        info!("Updated state invalid");
    }
}

fn remote_state_change_monitor(
    mut online_event_listener: EventReader<OnlineStateChange>,
    mut game_handler: ResMut<GameHandler>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for ev in online_event_listener.read() {
        info!("State: {:?}", game_handler.get_pushed_state());

        match game_handler.get_pushed_state() {
            StateUpdateRef::GameState(game_state) => {
                next_game_state.set(game_state); // This sets the next game state
            },
            StateUpdateRef::LevelState(_) => {
                info!("LevelState received, no state transition set for GameState.");
            },
        }
    }
}

fn receive_messages(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut game_handler: ResMut<GameHandler>,
    mut online_event_handler: EventWriter<OnlineStateChange>,
) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => {
                info!("Received message: {message:?}");
                // let split: &Vec<&str> = &message.split("::").collect();
                // for i in 0..split.len() {
                //     info!("split{}: {:?}", i, split[i]);
                // }
                server_parse_message(message, &mut game_handler, &mut online_event_handler);
            },
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}

/*
GameState                   MenuState                   PlayerCompletionState               LevelState
    #[default]                  #[default]                  #[default]                          #[default]
    LoadingScreen,              NoSelection,                NotInGame,                          MainMenu,
    Menus,                      Online,                     HoleIncomplete,                     Hole1,
    GameInitLocal,              Local,                      HoleCompleted,                      Hole2,
    GameInitOnline,             Tutorial,                                                       Hole3,
    InGame,                     LeaderBoard,                                                    Hole4,
    InGamePaused,               Preferences,                                                    Hole5,
    PostGameReview,             Player,                                                         Hole6,
    LeaderBoard,                                                                                Hole7,
    Preferences,                                                                                Hole8,
                                                                                                Hole9,
TurnState                   MapSetState                 PlayThroughStyleState                   Hole10,
    #[default]                  #[default]                  #[default]                          Hole11,
    Idle,                       Tutorial,                   Proximity,                          Hole12,
    Turn,                       WholeCorse,                 RandomSetOrder,                     Hole13,
    TurnReset,                  FrontNine,                                                      Hole14,
    NextTurn,                   BackNine,                                                       Hole15,
    HoleComplete,               SelectAHole,                                                    Hole16,
    GameComplete,                                                                               Hole17,
                                                                                                Hole18,
                                                                                                HoleTutorial
                                                                                                MenuLeaderBoard
LeaderBoardState            PartyConnectionState        Party {                                 MenuLocal
    #[default]                  #[default]                  players: Arc<[Player]>,             MenuOnline
    Mixed,                      Local,                      active_player: Arc<i32>,            MenuPreferences
    Online,                     Online,                     active_level: Arc<i32>,
    Local,                                              }
    PostGame,
    InGame,
    InGameOnline, 

CameraOrbitEntityState      Player {
    #[default]                  pub player_id: String,
    MainMenu,                   pub hole_completion_state: PlayerCompletionState,
    Ball,                       pub ball_material: Color, // For now custom material/shaders planned
    Cup,                        pub ball_location: Vec3,
    FreePan,                    pub puts_count_total: u32,
    LeaderBoard,                pub puts_hole_1: u32,
    GameInit.                   pub puts_hole_2: u32,
    MenuLocal,                  pub puts_hole_3: u32,
    MenuOnline,                 pub puts_hole_4: u32,
    MenuPreferences,            pub puts_hole_5: u32,
    MenuPlayer,                 pub puts_hole_6: u32,
                                pub puts_hole_7: u32,
                                pub puts_hole_8: u32,
                                pub puts_hole_9: u32,
                                pub puts_hole_10: u32,
                                pub puts_hole_11: u32,
                                pub puts_hole_12: u32,
                                pub puts_hole_13: u32,
                                pub puts_hole_14: u32,
                                pub puts_hole_15: u32,
                                pub puts_hole_16: u32,
                                pub puts_hole_17: u32,
                                pub puts_hole_18: u32,
*/