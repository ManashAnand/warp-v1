mod ai;
mod sys;
mod todo;

use clap::Parser;

use crate::ai::ai_response;
use crate::sys::{general_bool,kill_process_pid};

use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ques: Option<String>,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    ai: bool,

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
