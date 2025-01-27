use diesel::prelude::*;
use dotenvy::from_path;
use std::{ env, path::Path };

use crate::os::project_folder;

pub fn establish_connection() -> SqliteConnection {
    let base_path = project_folder();
    let env_path = Path::new(&base_path).join(".env");

    from_path(&env_path).ok();

    let database_url = env::var("DATABASE_URL").unwrap().to_string();
    SqliteConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}
