// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    input::common_conditions::*,
    // time::common_conditions::on_timer, 
    // utils::Duration,
    window::{PresentMode, WindowTheme},
};

// --- External Plugins --- //
// use bevy_editor_pls::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_rapier3d::prelude::*;

use std::sync::{
    Arc,
    Mutex,
};

// --- States --- //
use minigolf::{ 
    StateArrow,
    StateCameraOrbitEntity,
    StateGame,
    StateGameConnection,
    StateGamePlayStyle,
    StateLevel,
    StateMapSet,
    StateMenu,
    StateTurn,
};

// --- Resources --- //
use minigolf::{
    CameraHandler,
    Fonts,
    GameHandler,
    GolfBallTag,
    Party,
    Player,
    PlayerLocal,
    PlayerAi,
    PlayerRemote,
    RunTrigger,
};

// --- User Interface Import --- //
use minigolf::user_interface::camera_world::{
    setup_3d_camera,
    pan_orbit_camera, 
    state_camera_orbit_entity_logic,
};
use minigolf::user_interface::user_interface::{
    setup_ui,
    update_ui,
};

// // --- User Interface - Game State Handler Import --- //
// use minigolf::user_interface::game_handler::{
// };

// // --- User Interface - Turn Handler Import --- //
// use minigolf::user_interface::turn_handler::{
// };

// // --- User Interface - Menu Handler Import --- //
// use minigolf::user_interface::menu_handler::{
// };

// // --- Level Handler Import --- //
// use minigolf::level_handler::level_handler::{
// };

// // --- Physics Handler Import --- //
// use minigolf::level_handler::physics_handler::{
// };

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
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
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }
            ),
        ))

        // --- Additional Plugins --- //
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(EditorPlugin::default())
    
        // --- State Initialization --- //
        .insert_state(StateArrow::Idle)
        .insert_state(StateCameraOrbitEntity::Menu)
        .insert_state(StateGame::NotInGame)
        .insert_state(StateGameConnection::Local)
        .insert_state(StateGamePlayStyle::SetOrder)
        .insert_state(StateLevel::MainMenu)
        .insert_state(StateMapSet::Tutorial)
        .insert_state(StateMenu::MenuMainMenu)
        .insert_state(StateTurn::NotInGame)

        // --- Resource Initialization --- //
        .insert_resource(CameraHandler::new())
        .insert_resource(GameHandler::new())
        .insert_resource(Fonts::new())
        .insert_resource(Party::new())

        // --- Startup Systems Initialization --- //
        .add_systems(Startup, setup_3d_camera)
        .add_systems(Startup, setup_ui)

        // Camera //
        .add_systems(Update, state_camera_orbit_entity_logic)
        .add_systems(Update, pan_orbit_camera)
        .add_systems(Update, update_ui)

        // Dev Systems //        
        .insert_resource(RunTrigger::new())
        .add_systems(Update, active_player_set_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.active_player_set_ball_location()))
        .add_systems(Update, active_player_add_bonk.run_if(|run_trigger: Res<RunTrigger>|run_trigger.active_player_add_bonk()))
        .add_systems(Update, cycle_active_player.run_if(|run_trigger: Res<RunTrigger>|run_trigger.cycle_active_player()))
        .add_systems(Update, cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.cycle_camera()))
        .add_systems(Update, cycle_state_map_set.run_if(|run_trigger: Res<RunTrigger>|run_trigger.cycle_state_map_set()))
        .add_systems(Update, game_handler_get_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_get_active_ball_location()))
        .add_systems(Update, game_handler_reset_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_reset_active_ball_location()))
        .add_systems(Update, game_handler_set_active_ball_location.run_if(|run_trigger: Res<RunTrigger>|run_trigger.game_handler_set_active_ball_location()))
        .add_systems(Update, set_hole_completion_state_true.run_if(|run_trigger: Res<RunTrigger>|run_trigger.set_hole_completion_state_true()))
        .add_systems(Update, state_turn_next_player_turn.run_if(|run_trigger: Res<RunTrigger>|run_trigger.state_turn_next_player_turn()))
        .add_systems(Update, toggle_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.toggle_state_game()))
            
        .add_systems(Update, temp_interface);

    app.run();
}

fn temp_interface(
    mut run_trigger: ResMut<RunTrigger>,
    keys: Res<ButtonInput<KeyCode>>,
    party: Res<Party>,
    state_game: Res<State<StateGame>>,
) {
    if keys.just_released(KeyCode::Space) {
        info!("just_released: Space");  
        run_trigger.set_target("toggle_state_game", true);
    };
    if keys.just_released(KeyCode::KeyB) {
        info!("just_released: KeyB");  
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("active_player_add_bonk", true);
            },
            StateGame::NotInGame => {},
        };
    };
    if keys.just_released(KeyCode::KeyA) { // should trigger with new turn
        info!("just_released: KeyA");  
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("set_hole_completion_state_true", true);
            },
            StateGame::NotInGame => {},
        };
    };
    if keys.just_released(KeyCode::KeyC) {
        info!("just_released: KeyC");  
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("cycle_camera", true);
            },
            StateGame::NotInGame => {},
        };
    };
    if keys.just_released(KeyCode::KeyM) {
        info!("just_released: KeyM");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("cycle_map_set", true);
            },
        };
    };
    if keys.just_released(KeyCode::KeyN) {
        info!("just_released: KeyN");  
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("state_turn_next_player_turn", true);
            },
            StateGame::NotInGame => {},
        };
    };
    if keys.just_released(KeyCode::KeyP) {
        info!("just_released: KeyP");  
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("cycle_active_player", true);},
            StateGame::NotInGame => {
            },
        };
    };
    if keys.just_released(KeyCode::Numpad1) {
        info!("just_released: Numpad1");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.remove_last_player();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad3) {
        info!("just_released: Numpad3");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.remove_ai();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad7) {
        info!("just_released: Numpad7");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_local = PlayerLocal {
                    player_id: String::from(format!("PlayerLocal{}@email.com", owned_party_size + 1)),
                    hole_completion_state: false,
                    ball_material: Color::srgb(1.0, 0.0, 1.0),
                    ball_location: Vec3::new(0.0, 0.0, 0.0),
                    bonks_level: 0,
                    bonks_game: 0,
                };

                let new_player = Arc::new(Mutex::new(new_player_local));
                party.add_player(new_player);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad8) {
        info!("just_released: Numpad7");  
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_remote = PlayerRemote {
                    player_id: String::from(format!("PlayerRemote{}@email.com", owned_party_size + 1)),
                    hole_completion_state: false,
                    ball_material: Color::srgb(1.0, 0.0, 1.0),
                    ball_location: Vec3::new(0.0, 0.0, 0.0),
                    bonks_level: 0,
                    bonks_game: 0,
                };

                let new_player = Arc::new(Mutex::new(new_player_remote));
                party.add_player(new_player);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad9) {
        info!("just_released: Numpad9");   
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                let owned_party_size: i32 = party.get_party_size() as i32;
                let new_player_ai = PlayerAi::new();
                let new_player = Arc::new(Mutex::new(new_player_ai));
                party.add_player(new_player);
            },
        };
    };
}

fn active_player_set_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>
    // golf_ball_tag_query: Query<Entity, With<GolfBallTag>>,
) {
    info!("function: active_player_set_ball_location"); 
    // let owned_match = GolfBallTag(party.get_active_player().try_into().unwrap());
    // for golf_ball in golf_ball_tag_query.iter_mut() {
    //     owned_match => {
    if let Some(current_ball_location) = game_handler.get_active_ball_location() {
        info!("Setting active ball location: {:?}", current_ball_location);
        party.active_player_set_ball_location(current_ball_location);
    } else {
        info!("No ball location set for the active player, setting default ZERO");
        party.active_player_set_ball_location(Vec3::new(0.0, 0.0, 0.0));
    }
    //     },
    //     _ => {},
    // }
    run_trigger.set_target("active_player_set_ball_location", false);
}

fn active_player_add_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
) {
    info!("function: active_player_add_bonk"); 
    run_trigger.set_target("active_player_set_ball_location", true);
    run_trigger.set_target("game_handler_set_active_ball_location", true);
    party.active_player_add_bonk();
    run_trigger.set_target("active_player_add_bonk", false);
}

fn cycle_active_player(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
) {
    info!("function: cycle_active_player"); 
    party.next_set_order_player();
    run_trigger.set_target("game_handler_get_active_ball_location", true);
    run_trigger.set_target("cycle_active_player", false);
}

fn cycle_state_camera(
    mut run_trigger: ResMut<RunTrigger>,
    camera_orbit_entity_state: Res<State<StateCameraOrbitEntity>>,
    mut next_camera_orbit_entity_state: ResMut<NextState<StateCameraOrbitEntity>>,
) {
    info!("function: cycle_state_camera"); 
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
            info!("StateCameraOrbitEntity::LeaderBoard");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::LeaderBoard);
        },
        StateCameraOrbitEntity::LeaderBoard => {
            info!("StateCameraOrbitEntity::Menu");
            next_camera_orbit_entity_state.set(StateCameraOrbitEntity::Menu);
        },
    }
    run_trigger.set_target("cycle_state_camera", false);
    info!("post response: cycle_camera: {}", run_trigger.get("cycle_camera"));  
}

fn cycle_state_map_set(
    mut run_trigger: ResMut<RunTrigger>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
){
    info!("function: cycle_state_map_set"); 
    match state_map_set.get() {
        StateMapSet::Tutorial => {
            info!("StateMapSet::WholeCorse");
            next_state_map_set.set(StateMapSet::WholeCorse);
        },
        StateMapSet::WholeCorse => {
            info!("StateMapSet::FrontNine");
            next_state_map_set.set(StateMapSet::FrontNine);
        },
        StateMapSet::FrontNine => {
            info!("StateMapSet::BackNine");
            next_state_map_set.set(StateMapSet::BackNine);
        },
        StateMapSet::BackNine => {
            info!("StateMapSet::SelectAHole");
            next_state_map_set.set(StateMapSet::SelectAHole);
        },
        StateMapSet::SelectAHole => {
            info!("StateMapSet::Tutorial");
            next_state_map_set.set(StateMapSet::Tutorial);
        },
    };
    run_trigger.set_target("cycle_state_map_set", false);
    info!("post response: cycle_state_map_set: {}", run_trigger.get("cycle_state_map_set"));  
}

fn set_hole_completion_state_true(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut party: ResMut<Party>,
) {
    info!("function: set_hole_completion_state_true"); 
    match state_game.get() {
        StateGame::InGame => {
            party.active_player_set_hole_completion_state(true);
        },
        StateGame::NotInGame => {},
    };
    run_trigger.set_target("set_hole_completion_state_true", false);
}

fn state_turn_next_player_turn(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    mut game_handler: ResMut<GameHandler>,
    mut party: ResMut<Party>,
) {
    info!("function: state_turn_next_player_turn"); 
    match state_game.get() {
        StateGame::InGame => {
            run_trigger.set_target("game_handler_reset_active_ball_location", true);
            next_state_turn.set(StateTurn::NextTurn);
            run_trigger.set_target("cycle_active_player", true);
            run_trigger.set_target("game_handler_get_active_ball_location", true);
            next_state_turn.set(StateTurn::Active);
            run_trigger.set_target("state_turn_next_player_turn", false);
        },
        StateGame::NotInGame => {},
    };
    info!("post response: state_turn_next_player_turn");  
}

fn toggle_state_game(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
) {
    info!("function: toggle_state_game"); 
    info!("Current Game State: {:?}", state_game.get());
    match state_game.get() {
        StateGame::NotInGame => {
            info!("StateGame::InGame");
            next_state_game.set(StateGame::InGame);
            info!("StateTurn::Active");
            next_state_turn.set(StateTurn::Active);
        },
        StateGame::InGame => {
            info!("StateGame::NotInGame");
            next_state_game.set(StateGame::NotInGame);
            info!("StateTurn::NotInGame");
            next_state_turn.set(StateTurn::NotInGame);
        },
    };
    run_trigger.set_target("toggle_state_game", false);
    info!("post response: toggle_state_game: {}", run_trigger.get("toggle_state_game"));  
}

fn game_handler_get_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_get_active_ball_location, active_player: {:?}", party.get_active_player()); 
    game_handler.set_active_ball_location(party.active_player_get_ball_location());

    // game_handler.get_active_ball_location();
    run_trigger.set_target("game_handler_get_active_ball_location", false);
}

fn game_handler_reset_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_reset_active_ball_location"); 
    let owned_active_player = party.get_active_player();
    let owned_golf_ball = format!("ball{}", owned_active_player);
    if let Some(owned_golf_ball_location) = game_handler.get_active_ball_location() {
        game_handler.set_active_ball_location(Vec3::new(0.0, 0.0, 0.0));
        info!("game_handler.get_active_ball_location(): {:?}", game_handler.get_active_ball_location());
    }
    run_trigger.set_target("game_handler_reset_active_ball_location", false);
}

fn game_handler_set_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    mut party: ResMut<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_set_active_ball_location"); 
    let owned_active_player = party.get_active_player();
    let owned_golf_ball = format!("ball{}", owned_active_player);
    if let Some(owned_golf_ball_location) = game_handler.get_active_ball_location() {
        game_handler.set_active_ball_location(owned_golf_ball_location + Vec3::new(5.0, 5.0, 5.0));
        info!("{:?}", game_handler.get_active_ball_location());
    }

    // for (entity, name, transform) in scene_meshes.iter() {
    //     match name.as_str() {
    //         owned_golf_ball => {
    //             game_handler.set_active_ball_location(transform.translation);
    //         },
    //         _ => {},
    //     }
    // }
    run_trigger.set_target("game_handler_set_active_ball_location", false);
}

/*
pub enum StateArrow {
    #[default]
    Idle,
    DrawingArrow,
}

pub enum StateCameraOrbitEntity {
    #[default]
    Menu,
    Ball,
    Cup,
    FreePan,
    LeaderBoard,
}

pub enum StateLevel {
    #[default]
    MainMenu,
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
    HoleTutorial,
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

pub enum StateMapSet {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}

pub enum StateMenu {
    #[default]
    MenuMainMenu,
    MenuLeaderBoard,
    MenuLocal,
    MenuOnline,
    MenuPreferences,
    MenuPlayer,
}

pub enum StateGame {
    #[default]
    NotInGame,
    InGame,
}

pub enum StateGameConnection {
    #[default]
    Local,
    Online,
}

pub enum StateGamePlayStyle {
    #[default]
    SetOrder,
    Proximity,
}

pub enum StateTurn {
    #[default]
    NotInGame,
    Active,
    NextTurn,
}
*/





