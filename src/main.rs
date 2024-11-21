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
    ConnectionState,
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

// --- User Interface - Game State Handler Import --- //
use minigolf::user_interface::game_state_handler::{
    game_state_response_menus,
};

// --- User Interface - Turn Handler Import --- //
use minigolf::user_interface::turn_state_handler::{
    turn_state_response_hole_complete,
    turn_state_response_turn_reset,
};

// --- User Interface - Menu Handler Import --- //
use minigolf::user_interface::menu_state_handler::{
    menu_state_response_local,
    menu_state_response_online,
    menu_state_response_leader_board,
    menu_state_response_tutorial,
    menu_state_response_player,
    menu_state_response_preferences,
};

// --- Level Handler Import --- //
use minigolf::level_handler::level_handler::{
    init_level_glb, 
    setup_ground, 
    setup_light, 
    purge_glb_all,
    purge_rigid_bodies,
};

// --- Physics Handler Import --- //
use minigolf::level_handler::physics_handler::{
    add_physics_query_and_update_scene,
    asset_event_listener,
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
        .insert_state(ConnectionState::Local)
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
        .insert_resource(Player::new())

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
        .add_systems(OnExit(LevelState::MainMenu), purge_glb_all)
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
        .add_systems(OnExit(LevelState::Hole18), purge_glb_all)
        .add_systems(OnExit(LevelState::HoleTutorial), purge_glb_all)
        .add_systems(OnExit(LevelState::MenuLeaderBoard), purge_glb_all)
        .add_systems(OnExit(LevelState::MenuLocal), purge_glb_all)
        .add_systems(OnExit(LevelState::MenuOnline), purge_glb_all)
        .add_systems(OnExit(LevelState::MenuPreferences), purge_glb_all)
        .add_systems(OnExit(LevelState::MenuPlayer), purge_glb_all)
        
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
    app.add_event::<OnlineStateChange>()
        .add_systems(Startup, start_socket)
        .add_systems(Update, receive_messages)
        .add_systems(Update, remote_state_change_monitor)
        .add_systems(Update, auth_server_handshake
            .run_if(|game_handler: Res<GameHandler>|game_handler.is_not_connected())
            .run_if(on_timer(Duration::from_secs(5))))
        .add_systems(Update, entity_toggle_visbility
            .run_if(on_timer(Duration::from_secs(5)))
        );


    app.run();
}



// Control Entity Visbility
pub fn entity_toggle_visbility(
    mut commands: Commands,
    mut scene_meshes: Query<(Entity, &Name, &mut Visibility)>,
    party: Res<Party>,
) {
    info!("{:?}", party.get_party_size());
    for (entity, name, mut visibility) in &mut scene_meshes {
        let name_owned = name.as_str();
        match name_owned {
            "local_menu_ai_golfball_1" => {
                info!(
                    "Entity: {:?} Visibility: {:?}",
                    name,
                    visibility,
                );
                *visibility = Visibility::Hidden;
            },
            _ => {},
        }
    }
    // if Some(entity).is_some()  {
    //     let mut visibility = visibilities.get_mut(entity).unwrap();
    //     info!("{:?}", visibility);
    // };
}








// Matchbox Network integration
#[derive(Clone, Debug)]
enum NetworkInterface {
    InitHandshake,
    HandshakeConfirmed,

    PlayerCheckProfile,
    PlayerUpload,
    PlayerUpdate,

    RequestLeaderBoard,
    UpdateLeaderBoard,
    
    FriendsGet,
    FriendSearch,
    FriendAdd,
    FriendRemove,

    PartyGet,
    PartyAddFriend,
    PartyAddAI,
    PartyRemovePlayer,
    PartyJoin,
    PartyLeave,
    
    FindMatchInit,
    FindMatchCancel,

    InGameQuit,
    InGameBonk,
    InGameHoleCompletePlayer,
}

fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

fn auth_server_handshake(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    party: Res<Party>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
        
    // grab the reference to the main player
    let player = party.get_main_player();
        
    // grab the player id
    let player_id = player.get_id();

    for peer in peers {
        let message = format!{
            "{}",
            player_id,
        };
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

    if game_handler.is_not_connected() { // handle a new connection.
        let parsed_state = match split[0] {
            "ConnectionState" => match split[1] {
                "Online" => Some(StateUpdateRef::ConnectionState(ConnectionState::Online)),
                _ => None,
            }
            _ => None,
        };
        
        if let Some(state) = parsed_state {
            info!("Updated state valid: {:?}", state);
            game_handler.auth_server_handshake_received(Some(state));
            online_event_handler.send(OnlineStateChange);
        } else {
            info!("Updated state invalid");
        }
    } else {
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
            game_handler.auth_server_handshake_received(Some(state));
            online_event_handler.send(OnlineStateChange);
        } else {
            info!("Updated state invalid");
        }
    }
}

fn remote_state_change_monitor(
    mut online_event_listener: EventReader<OnlineStateChange>,
    mut game_handler: ResMut<GameHandler>,
    mut next_connection_state: ResMut<NextState<ConnectionState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for ev in online_event_listener.read() {
        let pushed_state =  game_handler.get_pushed_state();
        match pushed_state {
            StateUpdateRef::ConnectionState(connection_state) => {
                next_connection_state.set(connection_state); 
                game_handler.set_connected_true();
            },
            StateUpdateRef::GameState(game_state) => {
                next_game_state.set(game_state); 
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
LeaderBoardState            ConnectionState             Party {                                 MenuLocal
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