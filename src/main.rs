mod ai;

use clap::Parser;

use crate::ai::ai_response;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ques: String,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long, default_value_t = false, action=clap::ArgAction::Set)]
    ai: bool,
}

fn main() {
    let args = Args::parse();

    if args.ai {
        println!("{} ", args.ques);
        let res =  ai_response(&args.ques);
        println!("{:?}",res);
    } else {
        println!("AI is disabled");
    }
    match args.name {
        Some(name) => println!("Name: {}", name),
        None => println!("No name provided"),
    }
}
