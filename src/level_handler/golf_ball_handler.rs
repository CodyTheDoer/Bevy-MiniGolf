use bevy::prelude::*;

use bevy::scene::ron::de::Position;
use uuid::Uuid;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::game_handler;
use crate::player_handler::party_handler;
use crate::GameHandler;
// --- Resources --- //
use crate::{
    GLBStorageID, 
    GolfBall, 
    GolfBallHandler,
    GolfBallPosition,
    Interactable, 
    Party, 
    RunTrigger,
};

impl GolfBallHandler {
    pub fn new() -> Self {
        Self {
            golf_balls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Add a new player's golf ball to the handler
    pub fn add(&self, player_id: &Uuid) {
        let mut golf_balls = self.golf_balls.lock().unwrap();
        golf_balls.push( GolfBallPosition{uuid: player_id.to_owned(), position: Vec3::ZERO, last_position: Vec3::ZERO});
    }

    pub fn info_log(&self) {        
        info!("GolfBallHandler Info:");
        info!("__________________________________________:");
        info!("{:?}", self);
        info!("__________________________________________:");
    }

    // Remove a player from the connected players list
    pub fn remove(&self, player_id: &Uuid) {
        info!("GolfBallHandler remove:");
        self.info_log();
        let mut remove = false;
        let mut remove_idx: usize = 0;
        let mut golf_balls = self.golf_balls.lock().unwrap();
        for (idx, golf_ball) in golf_balls.iter().enumerate() {
            if golf_ball.uuid == player_id.to_owned() {
                remove_idx = idx;
                remove = true;
            }
        };
        if remove == true {
            golf_balls.remove(remove_idx);
        }
        self.info_log();
    }

    pub fn get_mutable(&mut self) -> &Arc<Mutex<Vec<GolfBallPosition>>> {
        &self.golf_balls
    }

    pub fn empty(&mut self) {
        let mut golf_balls = self.golf_balls.lock().unwrap();
        if golf_balls.len() > 0 {
            golf_balls.pop();
        };
    }

    // Reference the position Vec for the target player's golf ball
    pub fn position_get(&self, player_id: &Uuid) -> Option<Vec3> {
        let mut position_storage: Option<Vec3> = None;
        let golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter() {
            if golf_ball.uuid == player_id.to_owned() {
                position_storage = Some(golf_ball.position);
            }
        }
        position_storage
    }

    // Reference the last position Vec for the target player's golf ball
    pub fn last_position_get(&self, player_id: &Uuid) -> Option<Vec3> {
        let mut last_position_storage: Option<Vec3> = None;
        let golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter() {
            if golf_ball.uuid == player_id.to_owned() {
                last_position_storage = Some(golf_ball.last_position);
            }
        }
        last_position_storage
    }

    // Update the position Vec for the player's golf ball
    pub fn position_set(&mut self, player_id: &Uuid, position: Vec3) {
        info!("GolfBallHandler position_set:");
        let mut golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter_mut() {
            if golf_ball.uuid == player_id.to_owned() {
                golf_ball.position = position;
            }
        }
    }

    // Update the position Vec for the player's golf ball
    pub fn last_position_set(&mut self, player_id: &Uuid, position: Vec3) {
        info!("GolfBallHandler last_position_set:");
        let mut golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter_mut() {
            if golf_ball.uuid == player_id.to_owned() {
                golf_ball.last_position = position;
            }
        }
    }

    pub fn last_position_store_position(&mut self, player_id: &Uuid) {
        info!("GolfBallHandler last_position_store_position:");
        let mut golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter_mut() {
            if golf_ball.uuid == player_id.to_owned() {
                golf_ball.last_position = golf_ball.position;
            }
        }
    }

    pub fn position_restore_last_position(&mut self, player_id: &Uuid) {
        info!("GolfBallHandler position_restore_last_position:");
        let mut golf_balls = self.golf_balls.lock().unwrap();
        for golf_ball in golf_balls.iter_mut() {
            if golf_ball.uuid == player_id.to_owned() {
                golf_ball.position = golf_ball.last_position;
            }
        }
    }
}

// Helper: golf_ball_handler_spawn_golf_balls_for_party_members
fn golf_ball_handler_init_golf_ball_uuid(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    glb_storage: &Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    player_id: &Uuid,
) {
    if let Some(basic_golf_ball) = glb_storage.glb.get(25) {
        let basic_golf_ball_handle: Handle<Scene> = asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(basic_golf_ball.map),
        );
        let _scene_entities = commands
            .spawn(SceneBundle {
                scene: basic_golf_ball_handle.clone(),
                ..default()
            })
            .insert(Interactable)
            .insert(GolfBall(player_id.to_string()))
            .id(); 
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn golf_ball_handler_spawn_golf_balls_for_party_members(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
    golf_ball_handler: Res<GolfBallHandler>,
    asset_server: Res<AssetServer>,
    glb_storage: Res<GLBStorageID>, //Arc<[MapID]> //map: Arc<str>,
    party: Res<Party>,
) {
    let id_storage = party.all_players_id_into_vec();
    for player in id_storage.iter() {
        golf_ball_handler_init_golf_ball_uuid(
            &mut commands,
            &asset_server,
            &glb_storage,
            player,
        );
        golf_ball_handler.add(player);
    };
    run_trigger.set_target("golf_ball_handler_spawn_golf_balls_for_party_members", false);
    info!("post response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members"));  
}

pub fn golf_ball_handler_reset_golf_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_ball_handler: ResMut<GolfBallHandler>,
    party: Res<Party>,
) {
    info!("function: golf_ball_handler_reset_golf_ball_locations "); 
    {
        for player in party.all_players_id_into_vec().iter() {
            golf_ball_handler.position_set(player, Vec3::ZERO);
        }
    }
    run_trigger.set_target("golf_ball_handler_reset_golf_ball_locations", false);
    info!("post response: golf_ball_handler_reset_golf_ball_locations: {}", run_trigger.get("golf_ball_handler_reset_golf_ball_locations"));  
}

pub fn golf_ball_handler_end_game(
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_ball_handler: ResMut<GolfBallHandler>,
) {
    info!("function: golf_ball_handler_end_game "); 
    {
        golf_ball_handler.empty();
    }
    run_trigger.set_target("golf_ball_handler_end_game", false);
    info!("post response: golf_ball_handler_end_game: {}", run_trigger.get("golf_ball_handler_end_game"));  
}

pub fn golf_ball_handler_party_store_locations(
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_ball_handler: ResMut<GolfBallHandler>,
    party: Res<Party>,
) {
    info!("function: golf_ball_handler_party_store_locations "); 
    {
        for player in party.all_players_id_into_vec().iter() {
            golf_ball_handler.last_position_store_position(player);
        }
    }
    run_trigger.set_target("golf_ball_handler_party_store_locations", false);
    info!("post response: golf_ball_handler_party_store_locations: {}", run_trigger.get("golf_ball_handler_party_store_locations"));  
}

pub fn golf_ball_handler_active_player_manual_bonk(
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_ball_handler: ResMut<GolfBallHandler>,
    party: Res<Party>
) {
    info!("function: golf_ball_handler_active_player_manual_bonk "); 
    {
        let point = golf_ball_handler.position_get(&party.active_player_get_player_id()).unwrap() + Vec3::new(5.0, 5.0, 5.0);
        golf_ball_handler.position_set(&party.active_player_get_player_id(), point);
        run_trigger.set_target("golf_ball_handler_party_store_locations", true);
    }
    run_trigger.set_target("golf_ball_handler_active_player_manual_bonk", false);
    info!("post response: golf_ball_handler_active_player_manual_bonk: {}", run_trigger.get("golf_ball_handler_active_player_manual_bonk"));  
}

/*


pub fn game_handler_update_players_ref_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_ref_ball_locations"); 
    {
        info!("function: game_handler_update_players_ref_ball_locations, active_player: {:?}", party.get_active_player_index()); 
        game_handler.active_player_ball_location_set(party.active_player_get_ball_location());

        // game_handler.get_active_ball_location();
    }
    run_trigger.set_target("game_handler_update_players_ref_ball_locations", false);
    info!("post response: game_handler_update_players_ref_ball_locations: {}", run_trigger.get("game_handler_update_players_ref_ball_locations")); 
}

pub fn game_handler_update_players_reset_ref_ball_locations(
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    mut game_handler: ResMut<GameHandler>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_reset_ref_ball_locations"); 
    {
        // let owned_active_player = party.get_active_player_index();
        // // let owned_golf_ball = format!("ball{}", owned_active_player);
        // if let Some(owned_golf_ball_location) = game_handler.get_active_ball_location() {
        // }
        
        game_handler.active_player_ball_location_set(Vec3::new(0.0, 0.0, 0.0));
        info!("game_handler.active_player_ball_location_get(): {:?}", game_handler.active_player_ball_location_get());
    }
    run_trigger.set_target("game_handler_update_players_reset_ref_ball_locations", false);
    info!("post response: game_handler_update_players_reset_ref_ball_locations: {}", run_trigger.get("game_handler_update_players_reset_ref_ball_locations"));  
}

pub fn game_handler_update_players_store_current_ball_locations_to_ref(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    mut golf_ball_query: Query<&mut GolfBall, &Name, &Transform>,
    mut golf_ball_handler: ResMut<GolfBallHandler>,
    party: Res<Party>,
) {
    info!("function: game_handler_update_players_store_current_ball_locations_to_ref "); 
    {
        let mut golf_balls = golf_ball_handler.get_mutable().lock().unwrap();
        
        for (golf_ball, transform) in golf_ball_query.iter_mut() {
            let active_player = String::from(party.active_player_get_player_id());
            let golf_ball_id = &golf_ball.0;

            // info!("golf_ball: [{}], location: [{}]", golf_ball.0, golf_ball.0);

            if golf_ball_id == &active_player {
                if let Some(owned_golf_ball_location) = game_handler.active_player_ball_location_get() {
                    game_handler.active_player_ball_location_set(owned_golf_ball_location);
                    info!("{:?}", game_handler.active_player_ball_location_get());
                }
                // info!("golf_ball: [{}], location: [{}]", golf_ball.0, golf_ball.0);
            }
        }
    }
    run_trigger.set_target("game_handler_update_players_store_current_ball_locations_to_ref", false);
    info!("post response: game_handler_update_players_store_current_ball_locations_to_ref: {}", run_trigger.get("game_handler_update_players_store_current_ball_locations_to_ref"));  
}

pub fn game_handler_update_players_manual_static_bonk_current_ball(
    mut game_handler: ResMut<GameHandler>,
    mut run_trigger: ResMut<RunTrigger>,
    // party: Res<Party>,
    // scene_meshes: Query<(Entity, &Name, &Transform)>,
) {
    info!("function: game_handler_update_players_manual_static_bonk_current_ball "); 
    {    
        if let Some(owned_golf_ball_location) = game_handler.active_player_ball_location_get() {
            game_handler.active_player_ball_location_set(owned_golf_ball_location + Vec3::new(5.0, 5.0, 5.0));
            info!("{:?}", game_handler.active_player_ball_location_get());
        }
    }
    run_trigger.set_target("game_handler_update_players_manual_static_bonk_current_ball", false);
    info!("post response: game_handler_update_players_manual_static_bonk_current_ball: {}", run_trigger.get("game_handler_update_players_manual_static_bonk_current_ball"));  
}
*/

