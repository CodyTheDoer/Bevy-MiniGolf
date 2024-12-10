use bevy::prelude::*;

use std::thread;
use std::time::Duration;

use uuid::Uuid;

// States
use crate::{
    StateCameraOrbitEntity, 
    StateGame, 
    StateLevel, 
    StateMapSet, 
    StateMenu, 
    StatePanOrbit,
    StateTurn, 
    StateUpdateRef
};

// Resources
use crate::{
    GameHandler,
    GameRecord,
    Party,
    RunTrigger,
};

impl GameHandler {
    pub fn new() -> Self {
        GameHandler {
            current_level: 0,
            arrow_state: false,
            network_server_connection: false,
            remote_game: false,
            remotely_pushed_state: None,
            game_id: None,
        }
    }

    pub fn remote_game_get(&self) -> bool {
        self.remote_game
    }
    
    pub fn remote_game_set_false(&mut self) {
        self.remote_game = false;
    }
    
    pub fn remote_game_set_true(&mut self) {
        self.remote_game = true;
    }
    
    pub fn game_id_get(&mut self) -> Uuid {
        if self.game_id == None {
            self.game_id_gen();
        }
        self.game_id.unwrap() // .expect("GameHandler.get_game_id(): game_id get/gen failed")
    }

    pub fn game_id_gen(&mut self) {
        self.game_id = Some(Uuid::now_v7());
    }

    pub fn game_id_clear(&mut self) {
        self.game_id = None;
    }

    // Level Handling logic
    pub fn current_level_set_next_level(&mut self) {
        self.current_level += 1;
    }

    pub fn current_level_set_mapset_start(&mut self, map_set_state: StateMapSet) {
        let owned_map_state = map_set_state.clone();
        info!("owned_map_state: {:?}", owned_map_state);
        match map_set_state {
            StateMapSet::Tutorial => {
                self.current_level = 19;
            }, 
            StateMapSet::WholeCorse => {
                self.current_level = 1;
            },
            StateMapSet::FrontNine => {
                self.current_level = 1;
            },
            StateMapSet::BackNine => {
                self.current_level = 10;
            },
            StateMapSet::SelectAHole => {
            },
        }
    }

    // Level Handling logic
    pub fn current_level_get(&self) -> i32 {
        self.current_level
    }

    pub fn current_level_set(&mut self, level: i32) {
        self.current_level = level;
    }

    pub fn current_level_set_tutorial(&mut self) {
        self.current_level_set(19);
    }

    pub fn current_level_set_menu_main(&mut self) {
        self.current_level_set(0);
    }

    pub fn current_level_set_menu_learderboard(&mut self) {
        self.current_level_set(20);
    }

    pub fn current_level_set_menu_local(&mut self) {
        self.current_level_set(21);
    }

    pub fn current_level_set_menu_online(&mut self) {
        self.current_level_set(22);
    }

    pub fn current_level_set_menu_preferences(&mut self) {
        self.current_level_set(23);
    }

    pub fn current_level_set_menu_player(&mut self) {
        self.current_level_set(24);
    }

    // Bonk UI Logic
    pub fn arrow_state_get(&self) -> bool {
        self.arrow_state
    }

    pub fn arrow_state_set_true(&mut self) {
        self.arrow_state = true;
    }

    pub fn arrow_state_set_false(&mut self) {
        self.arrow_state = false;
    }
    
    // Remote Auth Server Logic
    pub fn is_connected(&self) -> bool {
        self.network_server_connection
    }
    
    pub fn is_not_connected(&self) -> bool {
        !self.network_server_connection
    }
    
    pub fn is_connected_set_false(&mut self) {
        self.network_server_connection = false;
    }
    
    pub fn is_connected_set_true(&mut self) {
        self.network_server_connection = true;
    }

    pub fn pushed_state_set(
        &mut self, 
        parsed_state: Option<StateUpdateRef>,
    ) {
        self.remotely_pushed_state = Some(parsed_state.unwrap());
    }

    pub fn pushed_state_get(&self) -> StateUpdateRef {
        self.remotely_pushed_state.clone().expect("Push State get failed.")
    }
}

impl GameRecord {
    pub fn unwrap(&self) -> (Uuid, Vec<Uuid>, Vec<[i32; 18]>) {
        (self.game_id, self.players.clone(), self.scores.clone())
    } 
}

pub fn game_handler_game_start (
    mut game_handler: ResMut<GameHandler>,
    mut next_level: ResMut<NextState<StateLevel>>,
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    state_map_set: Res<State<StateMapSet>>,
) {
    info!("function: game_handler_game_start "); 
    {
        if game_handler.remote_game_get() {

        } else {
            run_trigger.set_target("game_handler_game_state_start_routines", true);
            match state_game.get() {
                StateGame::NotInGame => {
                    match state_map_set.get() {
                        StateMapSet::Tutorial => {
                            game_handler.current_level_set(0);
                            next_level.set(StateLevel::HoleTutorial);
                        },
                        StateMapSet::WholeCorse => {
                            game_handler.current_level_set(1);
                            next_level.set(StateLevel::Hole1);
                        },
                        StateMapSet::FrontNine => {
                            game_handler.current_level_set(1);
                            next_level.set(StateLevel::Hole1);
                        },
                        StateMapSet::BackNine => {
                            game_handler.current_level_set(10);
                            next_level.set(StateLevel::Hole10);
                        },
                        StateMapSet::SelectAHole => {},
                    };
                    info!("level_handler_init_level_game_handler_current_level: level [{}]", game_handler.current_level_get());
                    run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                },
                StateGame::InGame => {
                    warn!("game_handler_game_start: FAILED! Game state already initiated!");
                },
            }; 
        }
    }
    run_trigger.set_target("game_handler_game_start", false);
    info!("post response: game_handler_game_start: {}", run_trigger.get("game_handler_game_start")); 
}

pub fn game_handler_game_state_exit_routines(
    mut run_trigger: ResMut<RunTrigger>,
    mut game_handler: ResMut<GameHandler>,
    mut party: ResMut<Party>,
    state_game: Res<State<StateGame>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut next_level: ResMut<NextState<StateLevel>>,
    mut next_menu_state: ResMut<NextState<StateMenu>>,
    mut next_camera_state: ResMut<NextState<StateCameraOrbitEntity>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
) {
    info!("function: game_handler_game_state_exit_routines"); 
    {    
        info!("Current Game State: {:?}", state_game.get());
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                run_trigger.set_target("golf_ball_handler_end_game", true);
                run_trigger.set_target("level_handler_purge_protocol", true);
                next_menu_state.set(StateMenu::MenuMainMenu);
                next_camera_state.set(StateCameraOrbitEntity::Menu);
                party.active_player_set(1);
                info!("StateGame::NotInGame");
                next_state_game.set(StateGame::NotInGame);
                info!("StateTurn::NotInGame");
                next_state_turn.set(StateTurn::NotInGame);
                game_handler.current_level_set(0);
                next_level.set(StateLevel::MainMenu);
                game_handler.current_level_set_menu_main();
                run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                party.game_completed();
                run_trigger.set_target("leader_board_review_last_game", true);
            },
        };
    }
    run_trigger.set_target("game_handler_game_state_exit_routines", false);
    info!("post response: game_handler_game_state_exit_routines: {}", run_trigger.get("game_handler_game_state_exit_routines"));  
}

pub fn game_handler_game_state_start_routines(
    mut run_trigger: ResMut<RunTrigger>,
    game_handler: Res<GameHandler>,
    state_game: Res<State<StateGame>>,
    mut next_state_game: ResMut<NextState<StateGame>>,
    mut next_camera_state: ResMut<NextState<StateCameraOrbitEntity>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
    mut pan_orbit_camera_query: Query<&mut StatePanOrbit>,
) {
    info!("function: game_handler_game_state_start_routines"); 
    {
        info!("Current Game State: {:?}", state_game.get());
        match state_game.get() {
            StateGame::NotInGame => {
                if game_handler.remote_game_get() {
                    info!("StateTurn::Idle");
                    next_state_turn.set(StateTurn::Idle);
                } else {
                    info!("StateTurn::Active");
                    next_state_turn.set(StateTurn::Active);
                }
                info!("StateGame::InGame");
                next_state_game.set(StateGame::InGame);
                next_camera_state.set(StateCameraOrbitEntity::Ball);
                for mut state in pan_orbit_camera_query.iter_mut() {
                    info!("{:?}", state);
                    state.radius = 2.75;
                    state.pitch = -15.0f32.to_radians();
                    state.yaw = 0.0f32.to_radians();
                };
            },
            StateGame::InGame => {},
        };
    }
    run_trigger.set_target("game_handler_game_state_start_routines", false);
    info!("post response: game_handler_game_state_start_routines: {}", run_trigger.get("game_handler_game_state_start_routines"));  
}