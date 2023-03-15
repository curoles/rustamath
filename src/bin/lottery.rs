use rustamath::random::lottery::lottery;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Throw lottery die
    Dice(DiceArgs),
    /// Play lottery
    Play(PlayArgs),
}

#[derive(Args)]
struct DiceArgs {
    /// Start number
    low: i64,

    /// Last number
    high: i64,

    /// Show first N
    #[arg(default_value = "0")]
    take: usize,
}

#[derive(Args)]
struct PlayArgs {
    /// Start number
    #[arg(default_value = "1")]
    low: u32,

    /// Last number
    #[arg(default_value = "59")]
    high: u32,

    /// Numbers to take
    #[arg(default_value = "6")]
    take: usize,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dice (args) => {
            dice(args.low, args.high, args.take)
        }
        Commands::Play (args) => {
            play(args.low, args.high, args.take)
        }
    }
}

fn dice(low: i64, high: i64, take: usize) {
    if high <= low {
        println!("Illegal range {}..{}", low, high);
        return;
    }

    let list = lottery(low, high);
    let len = (high - low + 1) as usize;
    let take = match take {
        0 => len,
        _ => if take > len {len} else {take},
    };

    println!("{:?}", &list[..take]);
}

//use inquire::{Text, validator::{StringValidator, Validation}};
use inquire::{CustomType, validator::Validation};

fn play(low: u32, high: u32, take: usize) {
    if high <= low {
        println!("Illegal range {}..{}", low, high);
        return;
    }

    let list = lottery(low, high);
    let len = (high - low + 1) as usize;
    let take = match take {
        0 => len,
        _ => if take > len {len} else {take},
    };

    //println!("{:?}", &list[..take]);

    let validator = move |input: &u32| if *input > high || *input < low {
        Ok(Validation::Invalid("Illegal number".into()))
    } else {
        Ok(Validation::Valid)
    };

    let mut user_list = Vec::<u32>::new();

    for i in 0..take {

        let user_list_clone = user_list.clone();
        let validator2 = move |input: &u32| if user_list_clone.contains(input) {
            Ok(Validation::Invalid("Number already in the list".into()))
        } else {
            Ok(Validation::Valid)
        };

        let num = CustomType::<u32>::new("Next number?")
            .with_validator(validator)
            .with_validator(validator2)
            .prompt();
        
        match num {
            Ok(num) => {
                user_list.push(num);
                println!("lottery: {:2?}", &list[..=i]);
                println!("you    : {:2?}", &user_list[..=i]);
            },
            Err(_) => {break},
        }
    }
}