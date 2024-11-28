use bevy::prelude::*;

use uuid::Uuid;

// States
use crate::{
    StateCameraOrbitEntity,
    StateLevel,
    StateGame,
    StateMapSet,
    StateMenu,
    StateTurn,
    StateUpdateRef,
};

// Resources
use crate::{
    GameHandler,
    Party,
    RunTrigger,
};

impl GameHandler {
    pub fn new() -> Self {
        GameHandler {
            current_level: 0,
            active_ball_location: None,
            arrow_state: false,
            network_server_connection: false,
            remotely_pushed_state: None,
            game_id: None,
        }
    }

    pub fn get_game_id(&mut self) -> Uuid {
        if self.game_id == None {
            self.gen_game_id();
        }
        self.game_id.unwrap() // .expect("GameHandler.get_game_id(): game_id get/gen failed")
    }

    pub fn gen_game_id(&mut self) {
        self.game_id = Some(Uuid::now_v7());
    }

    pub fn clear_game_id(&mut self) {
        self.game_id = None;
    }

    // Level Handling logic
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }

    pub fn set_starting_level(&mut self, map_set_state: StateMapSet) {
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
    pub fn game_completed(&mut self) {
        self.set_current_level(0);
        self.set_active_ball_location(Vec3::ZERO);
    }

    pub fn get_current_level(&self) -> i32 {
        self.current_level
    }

    pub fn set_current_level(&mut self, level: i32) {
        self.current_level = level;
    }

    pub fn get_active_ball_location(&mut self) -> Option<Vec3> {
        if let Some(ball_location) = self.active_ball_location.clone() {
            Some(ball_location)
        } else {
            self.set_active_ball_location(Vec3::ZERO);
            self.active_ball_location.clone()
        }
    }

    pub fn set_active_ball_location(&mut self, point: Vec3) {
        info!("function: set_active_ball_location: {:?}", point);
        self.active_ball_location = Some(point);
    }

    pub fn init_postgame_leaderboard(&mut self) {
        self.set_current_level(20);
    }

    pub fn init_tutorial(&mut self) {
        self.set_current_level(19);
    }

    pub fn init_menu_main(&mut self) {
        self.set_current_level(0);
    }

    pub fn init_menu_leader_board(&mut self) {
        self.set_current_level(20);
    }

    pub fn init_menu_local(&mut self) {
        self.set_current_level(21);
    }

    pub fn init_menu_online(&mut self) {
        self.set_current_level(22);
    }

    pub fn init_menu_preferences(&mut self) {
        self.set_current_level(23);
    }

    pub fn init_menu_player(&mut self) {
        self.set_current_level(24);
    }

    // Bonk UI Logic
    pub fn get_arrow_state(&self) -> bool {
        self.arrow_state
    }

    pub fn set_arrow_state_true(&mut self) {
        self.arrow_state = true;
    }

    pub fn set_arrow_state_false(&mut self) {
        self.arrow_state = false;
    }
    
    // Remote Auth Server Logic
    pub fn is_connected(&self) -> bool {
        self.network_server_connection
    }
    
    pub fn is_not_connected(&self) -> bool {
        if self.network_server_connection == false {
            true
        } else {
            false
        }
    }
    
    pub fn set_connected_false(&mut self) {
        self.network_server_connection = false;
    }
    
    pub fn set_connected_true(&mut self) {
        self.network_server_connection = true;
    }

    pub fn auth_server_handshake_received(
        &mut self, 
        parsed_state: Option<StateUpdateRef>,
    ) {
        self.remotely_pushed_state = Some(parsed_state.unwrap());
    }

    pub fn get_pushed_state(&self) -> StateUpdateRef {
        self.remotely_pushed_state.clone().expect("Push State get failed.")
    }
}

pub fn game_handler_cycle_current_level(
    mut run_trigger: ResMut<RunTrigger>,
    state_level: Res<State<StateLevel>>,
    mut next_level: ResMut<NextState<StateLevel>>,
) {
    info!("function: game_handler_cycle_current_level"); 
    match state_level.get() {
        StateLevel::Hole1 => {
            next_level.set(StateLevel::Hole2);
        },
        StateLevel::Hole2 => {
            next_level.set(StateLevel::Hole3);
        },
        StateLevel::Hole3 => {
            next_level.set(StateLevel::Hole4);
        },
        StateLevel::Hole4 => {
            next_level.set(StateLevel::Hole5);
        },
        StateLevel::Hole5 => {
            next_level.set(StateLevel::Hole6);
        },
        StateLevel::Hole6 => {
            next_level.set(StateLevel::Hole7);
        },
        StateLevel::Hole7 => {
            next_level.set(StateLevel::Hole8);
        },
        StateLevel::Hole8 => {
            next_level.set(StateLevel::Hole9);
        },
        StateLevel::Hole9 => {
            next_level.set(StateLevel::Hole10);
        },
        StateLevel::Hole10 => {
            next_level.set(StateLevel::Hole11);
        },
        StateLevel::Hole11 => {
            next_level.set(StateLevel::Hole12);
        },
        StateLevel::Hole12 => {
            next_level.set(StateLevel::Hole13);
        },
        StateLevel::Hole13 => {
            next_level.set(StateLevel::Hole14);
        },
        StateLevel::Hole14 => {
            next_level.set(StateLevel::Hole15);
        },
        StateLevel::Hole15 => {
            next_level.set(StateLevel::Hole16);
        },
        StateLevel::Hole16 => {
            next_level.set(StateLevel::Hole17);
        },
        StateLevel::Hole17 => {
            next_level.set(StateLevel::Hole18);
        },
        _ => {},
    };
    run_trigger.set_target("game_handler_cycle_current_level", false);
}


pub fn game_handler_cycle_state_camera(
    mut run_trigger: ResMut<RunTrigger>,
    camera_orbit_entity_state: Res<State<StateCameraOrbitEntity>>,
    mut next_camera_orbit_entity_state: ResMut<NextState<StateCameraOrbitEntity>>,
) {
    info!("function: game_handler_cycle_state_camera"); 
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
    run_trigger.set_target("game_handler_cycle_state_camera", false);
    info!("post response: game_handler_cycle_state_camera: {}", run_trigger.get("game_handler_cycle_state_camera"));  
}

pub fn game_handler_cycle_state_map_set(
    mut run_trigger: ResMut<RunTrigger>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
){
    info!("function: game_handler_cycle_state_map_set"); 
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
    run_trigger.set_target("game_handler_cycle_state_map_set", false);
    info!("post response: game_handler_cycle_state_map_set: {}", run_trigger.get("game_handler_cycle_state_map_set"));  
}


pub fn game_handler_get_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_get_active_ball_location, active_player: {:?}", party.get_active_player_index()); 
    game_handler.set_active_ball_location(party.active_player_get_ball_location());

    // game_handler.get_active_ball_location();
    run_trigger.set_target("game_handler_get_active_ball_location", false);
}

pub fn game_handler_reset_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_reset_active_ball_location"); 
    // let owned_active_player = party.get_active_player_index();
    // // let owned_golf_ball = format!("ball{}", owned_active_player);
    // if let Some(owned_golf_ball_location) = game_handler.get_active_ball_location() {
    // }
    
    game_handler.set_active_ball_location(Vec3::new(0.0, 0.0, 0.0));
    info!("game_handler.get_active_ball_location(): {:?}", game_handler.get_active_ball_location());
    run_trigger.set_target("game_handler_reset_active_ball_location", false);
}

pub fn game_handler_set_active_ball_location(
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_set_active_ball_location"); 
    // let owned_active_player = party.get_active_player_index();
    // let owned_golf_ball = format!("ball{}", owned_active_player);
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

pub fn game_handler_state_turn_next_player_turn(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    mut next_state_turn: ResMut<NextState<StateTurn>>,
) {
    info!("function: game_handler_state_turn_next_player_turn"); 
    match state_game.get() {
        StateGame::InGame => {
            run_trigger.set_target("game_handler_reset_active_ball_location", true);
            next_state_turn.set(StateTurn::NextTurn);
            run_trigger.set_target("party_handler_cycle_active_player", true);
            run_trigger.set_target("game_handler_get_active_ball_location", true);
            next_state_turn.set(StateTurn::Active);
        },
        StateGame::NotInGame => {},
    };
    run_trigger.set_target("game_handler_state_turn_next_player_turn", false);
    info!("post response: game_handler_state_turn_next_player_turn");  
}

pub fn game_handler_start_game_local(
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_level: ResMut<NextState<StateLevel>>,
    mut game_handler: ResMut<GameHandler>,
) {
    info!("function: game_handler_start_game_local"); 
    match state_game.get() {
        StateGame::NotInGame => {
            match state_map_set.get() {
                StateMapSet::Tutorial => {
                    game_handler.set_current_level(0);
                    next_level.set(StateLevel::HoleTutorial);
                },
                StateMapSet::WholeCorse => {
                    game_handler.set_current_level(1);
                    next_level.set(StateLevel::Hole1);
                },
                StateMapSet::FrontNine => {
                    game_handler.set_current_level(1);
                    next_level.set(StateLevel::Hole1);
                },
                StateMapSet::BackNine => {
                    game_handler.set_current_level(10);
                    next_level.set(StateLevel::Hole10);
                },
                StateMapSet::SelectAHole => {},
            };
            run_trigger.set_target("game_handler_toggle_state_game", true);
        },
        StateGame::InGame => {},
    };
    run_trigger.set_target("game_handler_start_game_local", false);
}

pub fn game_handler_toggle_state_game(
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
    info!("function: game_handler_toggle_state_game"); 
    info!("Current Game State: {:?}", state_game.get());
    match state_game.get() {
        StateGame::NotInGame => {
            info!("StateGame::InGame");
            next_state_game.set(StateGame::InGame);
            info!("StateTurn::Active");
            next_state_turn.set(StateTurn::Active);
            next_camera_state.set(StateCameraOrbitEntity::Ball)
        },
        StateGame::InGame => {
            next_level.set(StateLevel::MainMenu);
            next_menu_state.set(StateMenu::MenuMainMenu);
            next_camera_state.set(StateCameraOrbitEntity::Menu);
            party.set_active_player(1);
            info!("StateGame::NotInGame");
            next_state_game.set(StateGame::NotInGame);
            info!("StateTurn::NotInGame");
            next_state_turn.set(StateTurn::NotInGame);
            game_handler.game_completed();
            party.game_completed();
            run_trigger.set_target("leader_board_review_last_game", true);
        },
    };
    run_trigger.set_target("game_handler_toggle_state_game", false);
    info!("post response: game_handler_toggle_state_game: {}", run_trigger.get("game_handler_toggle_state_game"));  
}