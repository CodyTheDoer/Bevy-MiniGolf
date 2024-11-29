// --- Internal Bevy Plugins --- //
use bevy::{prelude::*,
    tasks::IoTaskPool,
};

use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use tokio::runtime::Runtime;
use uuid::Uuid;

// --- States --- //

// --- Resources --- //
use crate::{
    DatabasePool,
    Party,
};

pub fn database_startup_system(pool: Res<DatabasePool>) {
    println!("Database pool has been set up successfully");
}

pub async fn query_boot_system(
    pool: MySqlPool,
    // mut party: ResMut<Party>,
) {
    // Count how many players exist in the player_table
    let row: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM player_table")
        .fetch_one(&pool)
        .await
    {
        Ok(row) => row,
        Err(err) => {
            eprintln!("Failed to execute query: {:?}", err);
            return;
        }
    };
    println!("Number of rows: {}", row.0);

    if row.0 == 0 {
        // No players exist, so let's create a new one
        let player_id = Uuid::now_v7(); // Generates a new UUID v7 for player_id
        let email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "player@example.com".to_string());
        let username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "DefaultUsername".to_string());

        // Insert the new player into the player_table
        match sqlx::query("INSERT INTO player_table (player_id, username, email, created, updated) VALUES (UUID_TO_BIN(?), ?, ?, NOW(), NOW())")
            .bind(player_id.to_string())
            .bind(username)
            .bind(email)
            .execute(&pool)
            .await
        {
            Ok(result) => {
                println!("Inserted new player with ID: {:?}", player_id);
                // party.player_set_player_id(0, player_id);
            },
            Err(err) => {
                eprintln!("Failed to insert new player: {:?}", err);
            }
        }
    } else {
        // Player already exists, you could retrieve the player_id and set it in Party here
        let existing_player: (Vec<u8>,) = match sqlx::query_as("SELECT player_id FROM player_table LIMIT 1")
            .fetch_one(&pool)
            .await
        {
            Ok(row) => row,
            Err(err) => {
                eprintln!("Failed to retrieve existing player_id: {:?}", err);
                return;
            }
        };

        // Convert the binary `player_id` back to a Uuid
        if let Ok(player_id) = Uuid::from_slice(&existing_player.0) {
            println!("Setting existing player ID: {}", player_id);
            // Call to set the player ID in Party system
            // party.player_set_player_id(0, player_id);
        } else {
            eprintln!("Failed to convert player_id from binary to UUID");
        }
    }
}

pub fn first_time_boot_system(
    pool: Res<DatabasePool>,
    // party: Res<Party>,
) {
    let pool = pool.0.clone();
    
    // Use Bevy's IoTaskPool to run the async function in the background
    let task_pool = IoTaskPool::get();

    task_pool.spawn(async move {
        // Create a new Tokio runtime
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        
        // Run the query within the runtime's context
        runtime.block_on(async move {
            query_boot_system(pool).await;
        });
    })
    .detach();
}

pub async fn establish_connection() -> sqlx::Result<sqlx::Pool<sqlx::MySql>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Set the number of maximum connections in the pool
        .connect(&database_url)
        .await?;

    Ok(pool)
}