use diesel_migrations::{ EmbeddedMigrations, MigrationHarness };
use todo_app::{ cli, database::establish_connection };
use diesel_migrations::embed_migrations;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn main() {
    let connection = &mut establish_connection();
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    cli::handle();
}
