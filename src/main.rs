mod ai;
mod sys;
use clap::Parser;

use crate::ai::ai_response;
use crate::sys::general_bool;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ques: Option<String>,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    ai: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    mem: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    system: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    disk: bool,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    network: bool,
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

    if args.mem {
        general_bool(true, "mem");
    }
    if args.system {
        general_bool(true, "system");
    }
    if args.disk {
        general_bool(true, "disk");
    }
    if args.network {
        general_bool(true, "network");
    }
}
