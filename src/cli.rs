use clap::{ Arg, ArgAction, Command };
use diesel::SelectableHelper;
use tabled::settings::Style;
use tabled::Tabled;

use crate::database;
use crate::models::*;
use diesel::prelude::*;

use tabled::Table;

pub fn handle() {
    let matches = Command::new("todo")
        .about("todo app")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list").short_flag('l').long_flag("list").about("List all todos"))
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .long_flag("add")
                .about("Add a todo")
                .arg(
                    Arg::new("title")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("title of todo")
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .short('d')
                        .action(ArgAction::Set)
                        .default_value("New todo")
                        .num_args(1..)
                        .help("description of todo")
                )
        )
        .subcommand(
            Command::new("done")
                .about("Add a todo")
                .arg(
                    Arg::new("title")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("title of todo")
                        .required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            use crate::schema::todos::dsl::*;

            let database_connection = &mut database::establish_connection();
            let results: Vec<Todo> = todos
                .select(Todo::as_select())
                .filter(completed.eq(false))
                .load(database_connection)
                .expect("Error loading todos");

            #[derive(Tabled)]
            struct TodoTable {
                title: String,
                description: String,
                time: String,
            }

            let table_data = results
                .iter()
                .map(|todo| TodoTable {
                    title: todo.title.clone(),
                    description: todo.description.clone().unwrap_or("None".to_string()),
                    time: todo.created_at.unwrap().format("%Y-%m-%d %H:%M").to_string(),
                })
                .collect::<Vec<TodoTable>>();

            let mut table = Table::new(table_data);
            table.with(Style::ascii());

            println!("{}", table);
        }
        Some(("add", add_matches)) => {
            use crate::schema::todos::dsl::*;

            let new_title = add_matches.get_one::<String>("title").unwrap();
            let new_description = add_matches.get_one::<String>("description").unwrap();

            let new_todos = vec![NewTodo {
                title: &new_title,
                description: Some(&new_description),
                completed: false,
                created_at: Some(chrono::Utc::now().naive_utc()),
            }];

            let database_connection = &mut database::establish_connection();
            let result = diesel::insert_into(todos).values(new_todos).execute(database_connection);

            match result {
                Ok(_) => println!("Added todo"),
                Err(_) => println!("Error adding todo"),
            }
        }
        Some(("done", done_matches)) => {
            use crate::schema::todos::dsl::*;

            let database_connection = &mut database::establish_connection();

            let search_term = done_matches.get_one::<String>("title").unwrap();

            let result = diesel
                ::update(
                    todos
                        .filter(title.like(format!("%{}%", search_term)))
                        .filter(completed.eq(false))
                )
                .set(completed.eq(true))
                .execute(database_connection);

            match result {
                Ok(count) => println!("Updated {} todos", count),
                Err(_) => println!("Error updating todo"),
            }
        }
        _ => unreachable!(),
    }
}
