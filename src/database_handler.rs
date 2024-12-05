// --- Internal Bevy Plugins --- //
use bevy::prelude::*;

use dotenv::dotenv;
use std::env;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

use std::sync::Arc;
use std::sync::Mutex;

// --- States --- //

// --- Resources --- //
use crate::{
    DatabaseConnection,
    Party,
    UpdateIdResource,
};

impl DatabaseConnection {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).expect("Failed to open SQLite database");
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}

pub fn db_pipeline_init_local_player(
    db: Res<DatabaseConnection>,
    mut party: ResMut<Party>,
    mut update_id_res: ResMut<UpdateIdResource>,
) {
    info!("Init: db_pipeline_sync_local_player:");
    info!("update_id: {:?}", update_id_res.update_id);
    dotenv().ok();
    let conn = db.get_connection();
    let conn = conn.lock().unwrap(); // Lock the mutex

    if let Some(update_id) = update_id_res.update_id {
        // Update the player ID with the provided update_id from the server
        info!("Updating local player ID to the provided update ID: {:?}", update_id);

        // Check if there are any existing players in the database
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);

        info!("Count: {:?}", count.clone());
        if count > 0 {
            // Update the existing player ID with the new update_id
            conn.execute(
                "UPDATE player_table SET player_id = ?1 WHERE player_id = (SELECT player_id FROM player_table LIMIT 1)",
                rusqlite::params![update_id.to_string()],
            )
            .expect("Failed to update existing player with new ID");
            info!("Updated player ID in the database to: {:?}", update_id);
        } else {
            // If no existing player, insert a new one with the update_id
            let player_username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "default_user".to_string());
            let player_email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "default_email@example.com".to_string());
            conn.execute(
                "INSERT INTO player_table (player_id, username, email) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    update_id.to_string(),
                    player_username,
                    player_email,
                ],
            )
            .expect("Failed to insert new player with update ID");
            info!("Inserted new player with ID: {:?}", update_id);
        }

        // Set the player ID in the Party resource
        party.player_set_player_id(0, update_id);
    } else {
        // No update ID provided, proceed with default behavior
        // Check for existing players
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);

        if count == 0 {
            // Insert a new player if no players exist
            let player_id = Uuid::now_v7().to_string();
            let player_username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "default_user".to_string());
            let player_email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "default_email@example.com".to_string());
            conn.execute(
                "INSERT INTO player_table (player_id, username, email) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    player_id,
                    player_username,
                    player_email,
                ],
            )
            .expect("Failed to insert new player");
            println!("Inserted new player with ID: {:?}", Uuid::parse_str(&player_id));
            let new_uuid = Uuid::parse_str(&player_id).expect("Parse from String to Uuid failed");
            party.player_set_player_id(0, new_uuid);
        } else {
            // Retrieve the existing player ID from the database
            let player_id: String = conn
                .query_row("SELECT player_id FROM player_table LIMIT 1", [], |row| row.get(0))
                .expect("Failed to retrieve player_id");
            let existing_uuid = Uuid::parse_str(&player_id).expect("Parse from String to Uuid failed");
            println!("Existing player ID: {:?}", existing_uuid);

            // Set the existing player ID in the Party resource
            party.player_set_player_id(0, existing_uuid);
            println!("New player ID: {:?}", party.main_player_get_player_id());
        }
    }
}


pub fn db_pipeline_sync_local_player(
    db: &Res<DatabaseConnection>,
    party: &mut ResMut<Party>,
    update_id_res: &ResMut<UpdateIdResource>,
) {
    info!("Init: db_pipeline_sync_local_player:");
    info!("update_id: {:?}", update_id_res.update_id);
    dotenv().ok();
    let conn = db.get_connection();
    let conn = conn.lock().unwrap(); // Lock the mutex

    if let Some(update_id) = update_id_res.update_id {
        // Update the player ID with the provided update_id from the server
        info!("Updating local player ID to the provided update ID: {:?}", update_id);

        // Check if there are any existing players in the database
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);

        info!("Count: {:?}", count.clone());
        if count > 0 {
            // Update the existing player ID with the new update_id
            conn.execute(
                "UPDATE player_table SET player_id = ?1 WHERE player_id = (SELECT player_id FROM player_table LIMIT 1)",
                rusqlite::params![update_id.to_string()],
            )
            .expect("Failed to update existing player with new ID");
            info!("Updated player ID in the database to: {:?}", update_id);
        } else {
            // If no existing player, insert a new one with the update_id
            let player_username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "default_user".to_string());
            let player_email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "default_email@example.com".to_string());
            conn.execute(
                "INSERT INTO player_table (player_id, username, email) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    update_id.to_string(),
                    player_username,
                    player_email,
                ],
            )
            .expect("Failed to insert new player with update ID");
            info!("Inserted new player with ID: {:?}", update_id);
        }

        // Set the player ID in the Party resource
        party.player_set_player_id(0, update_id);
    } else {
        // No update ID provided, proceed with default behavior
        // Check for existing players
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);

        if count == 0 {
            // Insert a new player if no players exist
            let player_id = Uuid::now_v7().to_string();
            let player_username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "default_user".to_string());
            let player_email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "default_email@example.com".to_string());
            conn.execute(
                "INSERT INTO player_table (player_id, username, email) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    player_id,
                    player_username,
                    player_email,
                ],
            )
            .expect("Failed to insert new player");
            println!("Inserted new player with ID: {:?}", Uuid::parse_str(&player_id));
            let new_uuid = Uuid::parse_str(&player_id).expect("Parse from String to Uuid failed");
            party.player_set_player_id(0, new_uuid);
        } else {
            // Retrieve the existing player ID from the database
            let player_id: String = conn
                .query_row("SELECT player_id FROM player_table LIMIT 1", [], |row| row.get(0))
                .expect("Failed to retrieve player_id");
            let existing_uuid = Uuid::parse_str(&player_id).expect("Parse from String to Uuid failed");
            println!("Existing player ID: {:?}", existing_uuid);

            // Set the existing player ID in the Party resource
            party.player_set_player_id(0, existing_uuid);
            println!("New player ID: {:?}", party.main_player_get_player_id());
        }
    }
}
