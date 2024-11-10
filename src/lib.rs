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

#[derive(Asset, Component, Debug, TypePath)]
pub struct Interactable; 

#[derive(Clone, Resource)]
pub struct GLBStorageID {
    glb: Vec<String>,
}

impl GLBStorageID {
    pub fn new() -> Self {
        let mut glb: Vec<String> = Vec::new();
        let map_t = "glb/boilerplate_level_tutorial.glb".to_string();
        glb.push(map_t);
        let map_1: String = String::from("glb/boilerplate_level_1.glb");
        let map_2: String = String::from("glb/boilerplate_level_2.glb");
        let map_3: String = String::from("glb/boilerplate_level_3.glb");
        let map_4: String = String::from("glb/boilerplate_level_4.glb");
        let map_5: String = String::from("glb/boilerplate_level_5.glb");
        let map_6: String = String::from("glb/boilerplate_level_6.glb");
        let map_7: String = String::from("glb/boilerplate_level_7.glb");
        let map_8: String = String::from("glb/boilerplate_level_8.glb");
        let map_9: String = String::from("glb/boilerplate_level_9.glb");
        let map_10: String = String::from("glb/boilerplate_level_10.glb");
        let map_11: String = String::from("glb/boilerplate_level_11.glb");
        let map_12: String = String::from("glb/boilerplate_level_12.glb");
        let map_13: String = String::from("glb/boilerplate_level_13.glb");
        let map_14: String = String::from("glb/boilerplate_level_14.glb");
        let map_15: String = String::from("glb/boilerplate_level_15.glb");
        let map_16: String = String::from("glb/boilerplate_level_16.glb");
        let map_17: String = String::from("glb/boilerplate_level_17.glb");
        let map_18: String = String::from("glb/boilerplate_level_18.glb");
        glb.push(map_1);
        glb.push(map_2);
        glb.push(map_3);
        glb.push(map_4);
        glb.push(map_5);
        glb.push(map_6);
        glb.push(map_7);
        glb.push(map_8);
        glb.push(map_9);
        glb.push(map_10);
        glb.push(map_11);
        glb.push(map_12);
        glb.push(map_13);
        glb.push(map_14);
        glb.push(map_15);
        glb.push(map_16);
        glb.push(map_17);
        glb.push(map_18);
        info!("{:?}", &glb);
        GLBStorageID {
            glb,
        }
    }
}

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

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum LevelState {
    #[default]
    HoleTutorial,
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
}

#[derive(Clone, Resource)]
pub struct GLBPurgeID {
    glb: Vec<String>,
}

impl GLBPurgeID {
    pub fn new() -> Self {
        let glb: Vec<String> = Vec::new();
        GLBPurgeID {
            glb,
        }
    }
}

// --- User Interface --- CameraUI --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

// --- User Interface --- CameraWorld --- //

#[derive(Asset, Component, TypePath)]
pub struct CameraWorld;

// --- User Interface --- //

pub struct UserInterface {}

impl UserInterface {
    pub fn select_a_hole_widget() -> i32 {
        let target = 0;
        target
    }
}

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

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum MapSetState {
    #[default]
    Tutorial,
    WholeCorse,
    FrontNine,
    BackNine,
    SelectAHole,
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    LoadingScreen,
    MenuMain,
    MenuSettings,
    MenuOnline,
    InGame,
    PostGameReview,
}

#[derive(Resource)]
pub struct GameStateHandler {
    current_level: i32,
    maps_index: i32,
}

impl GameStateHandler {
    pub fn new() -> Self {
        let current_level = 0;
        let maps_index = 0;
        GameStateHandler {
            current_level,
            maps_index,
        }
    }
}
