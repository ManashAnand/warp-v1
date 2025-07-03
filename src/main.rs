mod ai;
mod sys;
mod todo;

use clap::Parser;

use crate::ai::ai_response;
use crate::sys::{general_bool, kill_process_pid};
use crate::todo::{add_task, delete_task, load_tasks, show_tasks, toggle_complition};

use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ques: Option<String>,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    todo: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    ai: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    specs: bool,

    #[arg(short, long)]
    kill_pid: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.ai {
        match args.ques {
            Some(ques) => {
                println!("{} ", ques);
                let res = ai_response(&ques).await;
                match res {
                    Ok(text) => println!("✅ Response: {}", text),
                    Err(e) => eprintln!("❌ Error: {}", e),
                }
            }
            None => println!("No ques provided"),
        }
    } else {
        println!("AI is disabled");
    }

    if args.specs {
        let options = vec![
            "commands",
            "Show memory specs",
            "Show system specs",
            "Show disk specs",
            "show network specs",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => println!("Working on commands"),
            1 => general_bool(true, "mem"),
            2 => general_bool(true, "system"),
            3 => general_bool(true, "disk"),
            4 => general_bool(true, "network"),
            _ => println!("Invalid selection"),
        }

        match args.kill_pid {
            Some(val) => kill_process_pid(val),
            None => {
                println!("");
            }
        }
    }

    let mut tasks = load_tasks();

    if args.todo {
        loop {
            let options = vec![
                "Show todos",
                "Add todos",
                "Edit completion",
                "delete todo",
                "Exit",
            ];
            let todo_option = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose todo action ")
                .default(0)
                .items(&options)
                .interact()
                .unwrap();

            match todo_option {
                0 => show_tasks(),
                1 => add_task(&mut tasks),
                2 => toggle_complition(&mut tasks),
                3 => delete_task(&mut tasks),
                4 => {
                    println!("Exit!");
                    break;
                }
                _ => println!("Invalid option"),
            }
        }
    }
}
