use bevy::prelude::*;

// States
use crate::{
    StateLevel,
    StateMapSet,
};

// Resources
use crate::RunTrigger;

pub fn level_handler_set_state_next_level(
    mut run_trigger: ResMut<RunTrigger>,
    state_level: Res<State<StateLevel>>,
    mut next_level: ResMut<NextState<StateLevel>>,
) {
    info!("function: level_handler_set_state_next_level"); 
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
    run_trigger.set_target("level_handler_set_state_next_level", false);
}

pub fn level_handler_set_state_next_map_set(
    mut run_trigger: ResMut<RunTrigger>,
    state_map_set: Res<State<StateMapSet>>,
    mut next_state_map_set: ResMut<NextState<StateMapSet>>,
){
    info!("function: level_handler_set_state_next_map_set"); 
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
    run_trigger.set_target("level_handler_set_state_next_map_set", false);
    info!("post response: level_handler_set_state_next_map_set: {}", run_trigger.get("level_handler_set_state_next_map_set"));  
}