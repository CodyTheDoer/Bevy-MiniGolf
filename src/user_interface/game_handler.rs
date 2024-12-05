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
    pub fn game_exit_reset_level_and_ball_location(&mut self) {
        self.current_level_set(0);
        self.active_player_ball_location_set(Vec3::ZERO);
    }

    pub fn current_level_get(&self) -> i32 {
        self.current_level
    }

    pub fn current_level_set(&mut self, level: i32) {
        self.current_level = level;
    }

    pub fn active_player_ball_location_get(&mut self) -> Option<Vec3> {
        if let Some(ball_location) = self.active_ball_location.clone() {
            Some(ball_location)
        } else {
            self.active_player_ball_location_set(Vec3::ZERO);
            self.active_ball_location.clone()
        }
    }

    pub fn active_player_ball_location_set(&mut self, point: Vec3) {
        info!("function: active_player_ball_location_set: {:?}", point);
        self.active_ball_location = Some(point);
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

pub fn game_handler_update_players_ref_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_ref_ball_locations, active_player: {:?}", party.get_active_player_index()); 
    game_handler.active_player_ball_location_set(party.active_player_get_ball_location());

    // game_handler.get_active_ball_location();
    run_trigger.set_target("game_handler_update_players_ref_ball_locations", false);
}

pub fn game_handler_update_players_reset_ref_ball_locations (
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_reset_ref_ball_locations "); 
    // let owned_active_player = party.get_active_player_index();
    // // let owned_golf_ball = format!("ball{}", owned_active_player);
    // if let Some(owned_golf_ball_location) = game_handler.get_active_ball_location() {
    // }
    
    game_handler.active_player_ball_location_set(Vec3::new(0.0, 0.0, 0.0));
    info!("game_handler.active_player_ball_location_get(): {:?}", game_handler.active_player_ball_location_get());
    run_trigger.set_target("game_handler_update_players_reset_ref_ball_locations ", false);
}

pub fn game_handler_update_players_store_current_ball_locations_to_ref (
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_store_current_ball_locations_to_ref "); 
    // let owned_active_player = party.get_active_player_index();
    // let owned_golf_ball = format!("ball{}", owned_active_player);
    if let Some(owned_golf_ball_location) = game_handler.active_player_ball_location_get() {
        game_handler.active_player_ball_location_set(owned_golf_ball_location + Vec3::new(5.0, 5.0, 5.0));
        info!("{:?}", game_handler.active_player_ball_location_get());
    }

    // for (entity, name, transform) in scene_meshes.iter() {
    //     match name.as_str() {
    //         owned_golf_ball => {
    //             game_handler.set_active_ball_location(transform.translation);
    //         },
    //         _ => {},
    //     }
    // }
    run_trigger.set_target("game_handler_update_players_store_current_ball_locations_to_ref ", false);
}

pub fn game_handler_game_start (
    mut run_trigger: ResMut<RunTrigger>,
    state_game: Res<State<StateGame>>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_level: ResMut<NextState<StateLevel>>,
    mut game_handler: ResMut<GameHandler>,
) {
    info!("function: game_handler_game_start "); 
    info!("If check here");
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
            run_trigger.set_target("game_handler_toggle_state_game", true);
        },
        StateGame::InGame => {},
    };
    run_trigger.set_target("game_handler_game_start ", false);
    todo!(); // Finish the if check for remote logic
}

pub fn game_handler_game_state_change_routines(
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
    info!("function: game_handler_game_state_change_routines"); 
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
            game_handler.game_exit_reset_level_and_ball_location();
            party.game_completed();
            run_trigger.set_target("leader_board_review_last_game", true);
        },
    };
    run_trigger.set_target("game_handler_game_state_change_routines", false);
    info!("post response: game_handler_game_state_change_routines: {}", run_trigger.get("game_handler_game_state_change_routines"));  
}