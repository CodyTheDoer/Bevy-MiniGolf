use bevy::prelude::*;

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
            all_sleeping: false,
            arrow_state: false,
            environment_loaded: false,
            golf_balls_loaded: false,
            golf_balls_bonk_trigger: true,
            golf_balls_store_location: true,
            in_game: false,
            round_start: true,
            network_server_connection: false,
            remote_game: false,
            current_level: 0,
            game_id: None,
            remotely_pushed_state: None,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "all_sleeping" => {
                self.all_sleeping
            },
            "arrow_state" => {
                self.arrow_state
            },
            "environment_loaded" => {
                self.environment_loaded
            },
            "golf_balls_loaded" => {
                self.golf_balls_loaded
            },
            "golf_balls_bonk_trigger" => {
                self.golf_balls_bonk_trigger
            },
            "golf_balls_store_location" => {
                self.golf_balls_store_location
            },
            "in_game" => {
                self.in_game
            },
            "round_start" => {
                self.round_start
            },
            "network_server_connection" => {
                self.network_server_connection
            },
            "remote_game" => {
                self.remote_game
            },
            _ => {
                warn!("Target: [{}] does not exist!!!", target); 
                false
            },
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "all_sleeping" => {
                self.all_sleeping = state;
                // info!("response: all_sleeping: {}", self.get("all_sleeping"));  
            },
            "arrow_state" => {
                self.arrow_state = state;
                info!("response: arrow_state: {}", self.get("arrow_state"));  
            },
            "environment_loaded" => {
                self.environment_loaded = state;
                info!("response: environment_loaded: {}", self.get("environment_loaded"));  
            },
            "golf_balls_loaded" => {
                self.golf_balls_loaded = state;
                info!("response: golf_balls_loaded: {}", self.get("golf_balls_loaded"));  
            },
            "golf_balls_bonk_trigger" => {
                self.golf_balls_bonk_trigger = state;
                info!("response: golf_balls_bonk_trigger: {}", self.get("golf_balls_bonk_trigger"));  
            },
            "golf_balls_store_location" => {
                self.golf_balls_store_location = state;
                info!("response: golf_balls_store_location: {}", self.get("golf_balls_store_location"));  
            },
            "in_game" => {
                self.in_game = state;
                info!("response: in_game: {}", self.get("in_game"));  
            },
            "round_start" => {
                self.round_start = state;
                info!("response: round_start: {}", self.get("round_start"));  
            },
            "network_server_connection" => {
                self.network_server_connection = state;
                info!("response: network_server_connection: {}", self.get("network_server_connection"));  
            },
            "remote_game" => {
                self.remote_game = state;
                info!("response: remote_game: {}", self.get("remote_game"));  
            },
            _ => {},
        }
    }

    pub fn all_sleeping(&self) -> bool {
        self.all_sleeping
    }

    pub fn arrow_state(&self) -> bool {
        self.arrow_state
    }

    pub fn environment_loaded(&self) -> bool {
        self.environment_loaded
    }

    pub fn golf_balls_loaded(&self) -> bool {
        self.golf_balls_loaded
    }

    pub fn golf_balls_bonk_trigger(&self) -> bool {
        self.golf_balls_bonk_trigger
    }

    pub fn golf_balls_store_location(&self) -> bool {
        self.golf_balls_store_location
    }

    pub fn in_game(&self) -> bool {
        self.in_game
    }

    pub fn round_start(&self) -> bool {
        self.round_start
    }

    pub fn network_server_connection(&self) -> bool {
        self.network_server_connection
    }

    pub fn remote_game(&self) -> bool {
        self.remote_game
    }

    // Game ID Logic
    
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

    pub fn current_level_set_mapset_start(&mut self, map_set_state: &StateMapSet) {
        let owned_map_state = map_set_state.clone();
        info!("owned_map_state: {:?}", owned_map_state);
        match map_set_state {
            &StateMapSet::ToBeSelected => {
            }, 
            &StateMapSet::Tutorial => {
                self.current_level = 19;
            }, 
            &StateMapSet::WholeCorse => {
                self.current_level = 1;
            },
            &StateMapSet::FrontNine => {
                self.current_level = 1;
            },
            &StateMapSet::BackNine => {
                self.current_level = 10;
            },
            &StateMapSet::SelectAHole => {
                self.current_level = 9;
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

    pub fn current_level_set_menu_main(&mut self) {
        self.current_level_set(0);
    }

    pub fn current_level_set_tutorial(&mut self) {
        self.current_level_set(19);
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

    // pushed state logic 

    pub fn pushed_state_get(&self) -> StateUpdateRef {
        self.remotely_pushed_state.clone().expect("Push State get failed.")
    }

    pub fn pushed_state_set(
        &mut self, 
        parsed_state: Option<StateUpdateRef>,
    ) {
        self.remotely_pushed_state = Some(parsed_state.unwrap());
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
        if game_handler.get("remote_game") {

        } else {
            let mut map_state_selected = false;
            run_trigger.set_target("game_handler_game_state_start_routines", true);
            match state_game.get() {
                StateGame::NotInGame => {
                    game_handler.set_target("in_game", true);
                    match state_map_set.get() {
                        StateMapSet::ToBeSelected => {
                        },
                        StateMapSet::Tutorial => {
                            map_state_selected = true;
                            game_handler.current_level_set_tutorial();
                            next_level.set(StateLevel::HoleTutorial);
                        },
                        StateMapSet::WholeCorse => {
                            map_state_selected = true;
                            game_handler.current_level_set(1);
                            next_level.set(StateLevel::Hole1);
                        },
                        StateMapSet::FrontNine => {
                            map_state_selected = true;
                            game_handler.current_level_set(1);
                            next_level.set(StateLevel::Hole1);
                        },
                        StateMapSet::BackNine => {
                            map_state_selected = true;
                            game_handler.current_level_set(10);
                            next_level.set(StateLevel::Hole10);
                        },
                        StateMapSet::SelectAHole => {},
                    };
                    if map_state_selected == true {
                        info!("level_handler_init_level_game_handler_current_level: level [{}]", game_handler.current_level_get());
                        run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                        run_trigger.set_target("game_handler_game_start", false);
                        info!("post response: game_handler_game_start: {}", run_trigger.get("game_handler_game_start")); 
                    };
                },
                StateGame::InGame => {
                    warn!("game_handler_game_start: FAILED! Game state already initiated!");
                    run_trigger.set_target("game_handler_game_start", false);
                    info!("post response: game_handler_game_start: {}", run_trigger.get("game_handler_game_start")); 
                },
            }; 
        }
    }
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
    mut next_map_set: ResMut<NextState<StateMapSet>>,
    mut pan_orbit_camera_query: Query<&mut StatePanOrbit>,
) {
    info!("function: game_handler_game_state_exit_routines"); 
    {    
        info!("Current Game State: {:?}", state_game.get());
        match state_game.get() {
            StateGame::NotInGame => {},
            StateGame::InGame => {
                game_handler.set_target("in_game", false);
                run_trigger.set_target("golf_ball_handler_end_game", true);
                run_trigger.set_target("level_handler_purge_protocol", true);
                next_menu_state.set(StateMenu::MenuMainMenu);
                party.active_player_set(1);
                next_state_game.set(StateGame::NotInGame);
                next_state_turn.set(StateTurn::NotInGame);
                next_map_set.set(StateMapSet::ToBeSelected);
                info!("StateGame::NotInGame");
                info!("StateTurn::NotInGame");
                game_handler.current_level_set(0);
                next_level.set(StateLevel::MainMenu);
                game_handler.current_level_set_menu_main();
                run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
                party.game_completed();
                run_trigger.set_target("leader_board_review_last_game", true);
                next_camera_state.set(StateCameraOrbitEntity::Menu);
                for mut state in pan_orbit_camera_query.iter_mut() {
                    info!("{:?}", state);
                    state.radius = 0.3;
                    state.pitch = -2.0f32.to_radians();
                    state.yaw = 0.0f32.to_radians();
                };
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
                if game_handler.get("remote_game") {
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
                    state.pitch = -17.5f32.to_radians();
                    state.yaw = 0.0f32.to_radians();
                };
            },
            StateGame::InGame => {},
        };
    }
    run_trigger.set_target("game_handler_game_state_start_routines", false);
    info!("post response: game_handler_game_state_start_routines: {}", run_trigger.get("game_handler_game_state_start_routines"));  
}

pub fn game_handler_start_tutorial(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut next_map_set_state: ResMut<NextState<StateMapSet>>,
) {
    info!("function: game_handler_start_tutorial"); 
    {
        game_handler.current_level_set_tutorial();
        run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
        next_map_set_state.set(StateMapSet::Tutorial);
        run_trigger.set_target("game_handler_game_start", true);
    }
    run_trigger.set_target("game_handler_start_tutorial", false);
    info!("post response: game_handler_start_tutorial: [{}]", run_trigger.get("game_handler_start_tutorial"));  
}

pub fn game_handler_start_local_back_nine(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut next_map_set_state: ResMut<NextState<StateMapSet>>,
) {
    info!("function: game_handler_start_local_back_nine"); 
    {
        game_handler.current_level_set(10);
        run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
        next_map_set_state.set(StateMapSet::BackNine);
        run_trigger.set_target("game_handler_game_start", true);
    }
    run_trigger.set_target("game_handler_start_local_back_nine", false);
    info!("post response: game_handler_start_local_back_nine: [{}]", run_trigger.get("game_handler_start_local_back_nine"));  
}

pub fn game_handler_start_local_front_nine(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut next_map_set_state: ResMut<NextState<StateMapSet>>,
) {
    info!("function: game_handler_start_local_front_nine"); 
    {
        game_handler.current_level_set(1);
        run_trigger.set_target("level_handler_init_level_game_handler_current_level", true);
        next_map_set_state.set(StateMapSet::FrontNine);
        run_trigger.set_target("game_handler_game_start", true);
    }
    run_trigger.set_target("game_handler_start_local_front_nine", false);
    info!("post response: game_handler_start_local_front_nine: [{}]", run_trigger.get("game_handler_start_local_front_nine"));  
}

pub fn game_handler_start_local_select_a_hole(
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("function: game_handler_start_local_select_a_hole"); 
    {
        
    }
    run_trigger.set_target("game_handler_start_local_select_a_hole", false);
    info!("post response: game_handler_start_local_select_a_hole: [{}]", run_trigger.get("game_handler_start_local_select_a_hole"));  
}

pub fn game_handler_start_local_whole_corse(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut next_level_state: ResMut<NextState<StateLevel>>,
    mut next_map_set_state: ResMut<NextState<StateMapSet>>,
) {
    info!("function: game_handler_start_local_whole_corse"); 
    {
        game_handler.current_level_set(1);
        next_level_state.set(StateLevel::Hole1);
        next_map_set_state.set(StateMapSet::WholeCorse);
        run_trigger.set_target("game_handler_game_start", true);
    }
    run_trigger.set_target("game_handler_start_local_whole_corse", false);
    info!("post response: game_handler_start_local_whole_corse: [{}]", run_trigger.get("game_handler_start_local_whole_corse"));  
}