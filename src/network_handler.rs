use bevy::prelude::*;
use bevy_matchbox::prelude::*;

use crate::{
    Party,
    RunTrigger,
    StateGame,
};

pub fn network_get_client_state_game(
    mut run_trigger: ResMut<RunTrigger>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut party: ResMut<Party>,
    state_game: Res<State<StateGame>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    let player_id = party.main_player_get_player_id();
    let state = state_game.get();
    for peer in peers {
        let message = format!{
            "{}::{:?}",
            player_id,
            state,
        };
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }

    
    run_trigger.set_target("network_get_client_state_game", false);
}