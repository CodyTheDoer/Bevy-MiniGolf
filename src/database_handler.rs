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

pub async fn database_establish_connection() -> sqlx::Result<sqlx::Pool<sqlx::MySql>> {
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

pub fn first_time_boot_system_local_player(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
) {
    let pool = pool.0.clone();

    // Spawn the background task using bevy_tokio_tasks
    runtime.spawn_background_task(move |ctx| {
        first_time_boot_local_player_creation(pool, ctx)
    });
}

pub fn first_time_boot_setup_map_set(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
) {
    let pool = pool.0.clone();

    // Spawn the background task using bevy_tokio_tasks
    runtime.spawn_background_task(move |ctx| {
        first_time_boot_setup_map_set_creation(pool, ctx)
    });
}

pub async fn first_time_boot_local_player_creation(
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

pub async fn first_time_boot_setup_map_set_creation(
    pool: MySqlPool,
    mut ctx: TaskContext, 
) {
    // Count how many players exist in the player_table
    let res: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM map_set_table")
        .fetch_one(&pool)
        .await
    {
        Ok(res) => res,
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

    println!("Number of res: {}", res.0);
    if res.0 == 0 {
        let _map_set_whole_couse = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Whole Course");
            let hole_range_start = 1;
            let hole_range_end = 18;
            let file_paths: [Option<&str>; 18] = [
                Some("glb/map/level_1.glb"), 
                Some("glb/map/level_2.glb"), 
                Some("glb/map/level_3.glb"), 
                Some("glb/map/level_4.glb"), 
                Some("glb/map/level_5.glb"), 
                Some("glb/map/level_6.glb"), 
                Some("glb/map/level_7.glb"), 
                Some("glb/map/level_8.glb"), 
                Some("glb/map/level_9.glb"), 
                Some("glb/map/level_10.glb"), 
                Some("glb/map/level_11.glb"), 
                Some("glb/map/level_12.glb"), 
                Some("glb/map/level_13.glb"), 
                Some("glb/map/level_14.glb"), 
                Some("glb/map/level_15.glb"), 
                Some("glb/map/level_16.glb"), 
                Some("glb/map/level_17.glb"), 
                Some("glb/map/level_18.glb"), 
            ];

            let insert_result = sqlx::query(
                "INSERT INTO map_set_table (map_set_id, last_updated, map_set_name, 
                        hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                        file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                        file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                        file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                        file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                    ) VALUES (
                        UUID_TO_BIN(?), NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                    )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
        
        let _map_set_front_nine = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Front Nine");
            let hole_range_start = 1;
            let hole_range_end = 9;
            let file_paths: [Option<&str>; 18] = [
                Some("glb/map/level_1.glb"), 
                Some("glb/map/level_2.glb"), 
                Some("glb/map/level_3.glb"), 
                Some("glb/map/level_4.glb"), 
                Some("glb/map/level_5.glb"), 
                Some("glb/map/level_6.glb"), 
                Some("glb/map/level_7.glb"), 
                Some("glb/map/level_8.glb"), 
                Some("glb/map/level_9.glb"), 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
            ];

            let insert_result = sqlx::query(
                "INSERT INTO map_set_table (map_set_id, last_updated, map_set_name, 
                        hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                        file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                        file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                        file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                        file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                    ) VALUES (
                        UUID_TO_BIN(?), NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                    )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
        
        let _map_set_back_nine = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Back Nine");
            let hole_range_start = 1;
            let hole_range_end = 18;
            let file_paths: [Option<&str>; 18] = [
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                Some("glb/map/level_10.glb"), 
                Some("glb/map/level_11.glb"), 
                Some("glb/map/level_12.glb"), 
                Some("glb/map/level_13.glb"), 
                Some("glb/map/level_14.glb"), 
                Some("glb/map/level_15.glb"), 
                Some("glb/map/level_16.glb"), 
                Some("glb/map/level_17.glb"), 
                Some("glb/map/level_18.glb"), 
            ];

            let insert_result = sqlx::query(
                "INSERT INTO map_set_table (map_set_id, last_updated, map_set_name, 
                        hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                        file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                        file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                        file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                        file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                    ) VALUES (
                        UUID_TO_BIN(?), NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                    )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
    };
}