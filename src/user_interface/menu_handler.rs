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

pub fn local_party_interface_ai_material_toggle(
    mut scene_meshes: Query<(Entity, &Name)>,
    party: Res<Party>,
    children_query: Query<(&Children, &Name)>,
    materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<&Handle<StandardMaterial>>,
) {
    if party.ai_vec.is_some() {
        let owned_ai_vec = party.ai_vec.as_ref().unwrap(); 
        let mut owned_entities_vec: Vec<Entity> = Vec::new();
        for (entity, name) in &mut scene_meshes {
            let name_owned = name.as_str();
            match name_owned {
                "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                    owned_entities_vec.push(entity);
                },
                _ => {},
            }
        }
        let mut vec_to_send: Vec<Entity> = Vec::new();
        for target in owned_ai_vec.iter() {
            vec_to_send.push(owned_entities_vec[target - 1]);
        } 
        if owned_entities_vec.len() > 0 {
            update_gltf_material_color(vec_to_send, children_query, materials, material_query);
        };
    }
}

fn update_current_mesh_color(
    index: u32,
) -> Color {
    match index {
        0 => { // Black
            Color::srgb(0.0, 0.0, 0.0)
        },
        1 => { // White
            Color::srgb(1.0, 1.0, 1.0)
        },
        2 => { // Red
            Color::srgb(1.0, 0.0, 0.0)
        },
        3 => { // Green
            Color::srgb(0.0, 1.0, 0.0)
        },
        4 => { // Blue
            Color::srgb(0.0, 0.0, 1.0)
        },
        _ => { // Gray
            Color::srgb(0.3, 0.3, 0.3)
        },
    }
}

fn update_gltf_material_color(
    entity_vec: Vec<Entity>,
    children_query: Query<(&Children, &Name)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    material_query: Query<&Handle<StandardMaterial>>,
) {
    for entity in entity_vec {
        info!("[{:?}]", entity);
        if let Ok((children, name)) = children_query.get(entity) {
            process_entity_children(
                &mut materials,
                &material_query,
                children,
                name,
                &children_query,
            );
        }
    }
}

fn process_entity_children(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    material_query: &Query<&Handle<StandardMaterial>>,
    children: &Children,
    name: &Name,
    children_query: &Query<(&Children, &Name)>,
) {
    for &child in children.iter() {
        let name_owned = name.as_str();
        match name_owned {
            "local_menu_players_golfball_2" | "local_menu_players_golfball_3" | "local_menu_players_golfball_4" | "local_menu_players_golfball_5" | "local_menu_players_golfball_6" => {
                if let Ok(material_handle) = material_query.get(child) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.base_color = update_current_mesh_color(1);
                    }
                }
            },
            _ => {},
        }

        // Recursively check grandchildren
        if let Ok((grandchildren, name)) = children_query.get(child) {
            process_entity_children(
                materials,
                material_query,
                grandchildren,
                name,
                children_query,
            );
        }
    }
}