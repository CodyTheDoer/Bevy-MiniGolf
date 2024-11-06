use bevy::prelude::*;

pub mod leaderboard_handler;
pub mod level_handler;
pub mod network_handler;
pub mod player_handler;
pub mod user_interface;

#[derive(Resource)]
pub struct Fonts {
    pub fonts: Vec<TextStyle>,
}

impl Fonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        Fonts {
            fonts,
        }
    }
}

#[derive(Clone, Resource)]
pub struct OpIndex {
    pub ui_entities: u32,
    pub state_info_call: u32,
}

impl OpIndex {
    pub fn new() -> Self {
        let ui_entities: u32 = 0;
        let state_info_call: u32 = 0;
        OpIndex {
            ui_entities,
            state_info_call,
        }
    }

    pub fn add_ui_entity(&mut self) {
        self.ui_entities += 1;
    }
}

// --- Level Handler --- //

#[derive(Component)]
pub struct Ground;

#[derive(Asset, Component, TypePath)]
pub struct Interactable; 

#[derive(Clone, Debug)]
pub enum InteractableEntities {
    Ground,
    Ent1,
    Ent2,
    Ent3,
    Ent4,
    Ent5,
    Ent6,
}

// --- User Interface --- CameraUI --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --- CameraWorld --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

// --- User Interface --- //

#[derive(Clone, Debug)]
pub enum InfoCall {
    Call0,
    Call1,
    Call2,
    Call3,
    Call4,
    Call5,
    Call6,
}
