use std::env;
use dotenv::dotenv;
use structopt::StructOpt;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

mod utils;
use utils::{
    add::add_todo, 
    complete::complete_todo,
    list::list_todos,
    remove::remove_todo,
    update::update_todo,
};

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { title: String },
    Remove {id: i64},
    Done { id: i64 },
    Update { id: i64, title: String},
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()>{
    dotenv().ok();
    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect_with
        (SqliteConnectOptions::new().
         filename(&env::var("DATABASE_URL")?).
         create_if_missing(true)).await?;

    sqlx::migrate!().run(&pool).await?;

    match args.cmd {
        Some(Command::Add { title }) => {
            println!("Adding new todo with title '{}'", title);
            let todo_id = add_todo(&pool, title).await?;
            println!("Added new todo with id {}", todo_id);
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {} as done", id);
            if complete_todo(&pool, id).await? {
                println!("Todo {} is marked as done", id);
                println!("Printing list of all todos");
                list_todos(&pool).await?;
            } else {
                println!("Invalid id {}", id);
            }
        }
        Some(Command::Remove { id }) => {
            println!("Deleting todo with id: {}", id);
            if remove_todo(&pool, id).await? {
                println!("Todo with id: {} is deleted", id);
                println!("Printing list of all todos");
                list_todos(&pool).await?;
            } else {
                println!("Invalid id {}", id);
            }
        }
        Some(Command::Update { id, title }) => {
            println!("Updating todo with id: {}", id);
            if update_todo(&pool, id, &title).await? {
                println!("Todo with id: {} is updated with title: {}", id, &title);
                println!("Printing list of all todos");
                list_todos(&pool).await?;
            } else {
                println!("Invalid id {}", id);
            }
        }
        None => {
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        }
    }

    Ok(())
}
