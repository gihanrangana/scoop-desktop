pub async fn initialize_database(app_handle: AppHandle) -> Result<(), String> {
    let db = app_handle
        .state::<tauri_plugin_sql::SqlitePool>()
        .get()
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS buckets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            url TEXT NOT NULL
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .await
    .map_err(|e| format!("Failed to create buckets table: {}", e))?;

    // Fetch buckets
    println!("Fetching available buckets...");

    match github_api::get_available_buckets().await {
        Ok(buckets) => {
            if let Some(buckets_obj) = buckets.as_object() {
                println!("Found {} buckets", buckets_obj.len());

                let tx = db
                    .begin()
                    .await
                    .map_err(|e| format!("Failed to begin transaction: {}", e))?;

                for (name, bucket) in buckets_obj {
                    if let Some(url_str) = url.as_str() {
                        tx.execute(
                            "INSERT OR REPLACE INTO buckets (name, url, description) VALUES (?, ?, ?)",
                            {
                                // Extract the last two segments of the URL for description
                                let path_segments: Vec<&str> = url_str.split('/').filter(|s| !s.is_empty()).collect();
                                let segment_count = path_segments.len();
                                let description = if segment_count >= 2 {
                                    format!("{}/{}", path_segments[segment_count-2], path_segments[segment_count-1])
                                } else if segment_count == 1 {
                                    path_segments[0].to_string()
                                } else {
                                    "".to_string()
                                };

                                (name, url_str, description)
                            },
                        )
                        .await
                        .map_err(|e| format!("Failed to insert bucket {}: {}", name, e))?;

                        println!("Inserted bucket: {}", name);
                    } else {
                        println!("Warning: URL for bucket {} is not a string", name);
                    }
                }

                // Commit the transaction
                tx.commit()
                    .await
                    .map_err(|e| format!("Failed to commit transaction: {}", e))?;

                println!("Buckets stored in database successfully");
                Ok(())
            } else {
                Err("Buckets data is not in expected format".to_string())
            }
        }
        Err(e) => Err(format!("Failed to fetch buckets: {}", e)),
    }
}
