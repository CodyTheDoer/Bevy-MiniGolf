use bevy::prelude::*;

// Resources
use crate::Party;

pub fn local_party_interface_visibliity_toggle(
    // mut commands: Commands,
    mut scene_meshes: Query<(&Name, &mut Visibility)>,
    party: Res<Party>,
) {
    let party_size = party.get_count_party();
    let ai_count = party.get_count_ai();
    for (name, mut visibility) in &mut scene_meshes {
        let name_owned = name.as_str();
        match party_size {
            1 => {
                match name_owned {
                    "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            2 => {
                match name_owned {
                    "local_menu_players_golfball_2" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            3 => {
                match name_owned {
                    "local_menu_players_golfball_2" | "local_menu_players_golfball_3" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            4 => {
                match name_owned {
                    "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            5 => {
                match name_owned {
                    "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            6 => {
                match name_owned {
                    "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            _ => {}
        }
        match ai_count {
            0 => {
                match name_owned {
                    "local_menu_ai_golfball_1" | "local_menu_ai_golfball_2" | "local_menu_ai_golfball_3" | "local_menu_ai_golfball_4" | "local_menu_ai_golfball_5" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            1 => {
                match name_owned {
                    "local_menu_ai_golfball_5" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_ai_golfball_1" | "local_menu_ai_golfball_2" | "local_menu_ai_golfball_3" | "local_menu_ai_golfball_4" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            2 => {
                match name_owned {
                    "local_menu_ai_golfball_5" | "local_menu_ai_golfball_4" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_ai_golfball_1" | "local_menu_ai_golfball_2" | "local_menu_ai_golfball_3" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            3 => {
                match name_owned {
                    "local_menu_ai_golfball_5" | "local_menu_ai_golfball_4" | "local_menu_ai_golfball_3" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_ai_golfball_1" | "local_menu_ai_golfball_2" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            4 => {
                match name_owned {
                    "local_menu_ai_golfball_5" | "local_menu_ai_golfball_4" | "local_menu_ai_golfball_3" | "local_menu_ai_golfball_2" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    "local_menu_ai_golfball_1" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Visible => {*visibility = Visibility::Hidden},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            5 => {
                match name_owned {
                    "local_menu_ai_golfball_5" | "local_menu_ai_golfball_4" | "local_menu_ai_golfball_3" | "local_menu_ai_golfball_2" | "local_menu_ai_golfball_1" => {
                        match *visibility {
                            Visibility::Inherited | Visibility::Hidden=> {*visibility = Visibility::Visible},
                            _ => {},
                        };
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }
}
