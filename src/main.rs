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
    Party,
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
        .add_systems(Update, cycle_state_camera.run_if(|run_trigger: Res<RunTrigger>|run_trigger.get("cycle_camera")))
        .add_systems(Update, cycle_state_map_set.run_if(|run_trigger: Res<RunTrigger>|run_trigger.get("cycle_state_map_set")))
        .add_systems(Update, toggle_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.get("toggle_game_state")))
            
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
        run_trigger.set_target("toggle_state_game", true);
    };
    if keys.just_released(KeyCode::KeyC) {
        match state_game.get() {
            StateGame::InGame => {
                run_trigger.set_target("cycle_camera", true);
            },
            StateGame::NotInGame => {},
        };
    };
    if keys.just_released(KeyCode::KeyM) {
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                run_trigger.set_target("cycle_map_set", true);
            },
        };
    };
    if keys.just_released(KeyCode::Numpad7) {
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.add_player();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad1) {
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.remove_player();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad9) {
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.add_ai();
            },
        };
    };
    if keys.just_released(KeyCode::Numpad3) {
        match state_game.get() {
            StateGame::InGame => {},
            StateGame::NotInGame => {
                party.remove_ai();
            },
        };
    };
}

fn cycle_state_camera(
    camera_orbit_entity_state: Res<State<StateCameraOrbitEntity>>,
    mut next_camera_orbit_entity_state: ResMut<NextState<StateCameraOrbitEntity>>,
    mut run_trigger: ResMut<RunTrigger>,
) {
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
}

fn cycle_state_map_set(
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
    mut run_trigger: ResMut<RunTrigger>,
){
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
}

fn toggle_state_game(
    state_game: Res<State<StateGame>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    match state_game.get() {
        StateGame::NotInGame => {
            info!("StateGame::InGame");
            next_state_game.set(StateGame::InGame);
        },
        StateGame::InGame => {
            info!("StateGame::NotInGame");
            next_state_game.set(StateGame::NotInGame);
        },
    };
    run_trigger.set_target("toggle_state_game", false);
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
    Idle,
    Active,
}
*/





