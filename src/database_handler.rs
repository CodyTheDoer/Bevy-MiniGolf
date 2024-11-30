// --- Internal Bevy Plugins --- //
use bevy::prelude::*;

use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use uuid::Uuid;

// --- States --- //

// --- Resources --- //
use crate::{
    DatabasePool,
    Party,
};

use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};

pub async fn query_boot_system(
    pool: MySqlPool,
    mut ctx: TaskContext, 
) {
    // Count how many players exist in the player_table
    let row: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM player_table")
        .fetch_one(&pool)
        .await
    {
        Ok(row) => row,
        Err(err) => {
            eprintln!("Failed to execute query: {:?}", err);
            // Run a callback on the main thread to handle the error properly
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err);
            })
            .await;
            return;
        }
    };

    println!("Number of rows: {}", row.0);
    let player_id = if row.0 == 0 {
        // No players exist, so let's create a new one
        let player_id = Uuid::now_v7(); // Generates a new UUID v7 for player_id
        let email = env::var("PLAYER_EMAIL").unwrap_or_else(|_| "player@example.com".to_string());
        let username = env::var("PLAYER_USERNAME").unwrap_or_else(|_| "DefaultUsername".to_string());

        // Insert the new player into the player_table
        match sqlx::query("INSERT INTO player_table (player_id, username, email, created, updated) VALUES (UUID_TO_BIN(?), ?, ?, NOW(), NOW())")
            .bind(player_id.to_string())
            .bind(&username)
            .bind(&email)
            .execute(&pool)
            .await
        {
            Ok(_) => {
                println!("Inserted new player with ID: {:?}", player_id);
                player_id
            },
            Err(err) => {
                eprintln!("Failed to insert new player: {:?}", err);
                // Run a callback on the main thread to handle the error properly
                ctx.run_on_main_thread(move |_ctx| {
                    info!("Failed to insert new player in the task: {:?}", err);
                })
                .await;
                return;
            }
        }
    } else {
        // Retrieve existing player ID
        let existing_player: (Vec<u8>,) = match sqlx::query_as("SELECT player_id FROM player_table LIMIT 1")
            .fetch_one(&pool)
            .await
        {
            Ok(row) => row,
            Err(err) => {
                eprintln!("Failed to retrieve existing player_id: {:?}", err);
                // Run a callback on the main thread to handle the error properly
                ctx.run_on_main_thread(move |_ctx| {
                    info!("Failed to retrieve existing player_id in the task: {:?}", err);
                })
                .await;
                return;
            }
        };

        // Convert the binary player_id back to a Uuid
        match Uuid::from_slice(&existing_player.0) {
            Ok(player_id) => {
                println!("Retrieved existing player ID: {}", player_id);
                player_id
            }
            Err(_) => {
                eprintln!("Failed to convert player_id from binary to UUID");
                ctx.run_on_main_thread(|_ctx| {
                    info!("Failed to convert player_id from binary in the task");
                })
                .await;
                return;
            }
        }
    };

    // Run on the main thread to update the resource with player ID
    ctx.run_on_main_thread(move |ctx| {
        info!("Successfully retrieved or created player ID: {:?}", player_id);
        if let Some(mut party) = ctx.world.get_resource_mut::<Party>() {
            // Update the Party resource as needed
            party.player_set_player_id(0, player_id);
            info!("Updated Party resource with player ID: {:?}", player_id);
        } else {
            info!("Failed to access Party resource");
        }
    })
    .await;
}

pub fn first_time_boot_system(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
) {
    let pool = pool.0.clone();

    // Spawn the background task using bevy_tokio_tasks
    runtime.spawn_background_task(move |ctx| {
        query_boot_system(pool, ctx)
    });
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