use bevy::prelude::*;

use uuid::Uuid;
// --- Resources --- //
use crate::{
    GLBStorageID, GolfBallTag, Interactable, Party, RunTrigger
};

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
            .insert(GolfBallTag(player_id.to_string()))
            .id(); 
    } else {
        warn!("Target was not valid. Refer to the GLBStorageID map in the library.");
    };
}

pub fn golf_ball_handler_spawn_golf_balls_for_party_members(
    mut commands: Commands,
    mut run_trigger: ResMut<RunTrigger>,
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
    };
    run_trigger.set_target("golf_ball_handler_spawn_golf_balls_for_party_members", false);
    info!("post response: golf_ball_handler_spawn_golf_balls_for_party_members: {}", run_trigger.get("golf_ball_handler_spawn_golf_balls_for_party_members"));  
}