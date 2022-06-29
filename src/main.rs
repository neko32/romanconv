use std::env;
use romanconv::roman_num::{single_flow_with_input, ResultHistory, load_history, persist_history};
use clap::Parser;
use std::ops::RangeInclusive;

const ROUND_RANGE:RangeInclusive<u16> = 1..=1000;
const RECORD_FPATH:&str = "/appstore/romanconv/data/data.json";

fn main() -> Result<(), anyhow::Error> {

    let args = Args::parse();

    let mut counter = 1;
    let mut correct = 0;
    let mut incorrect = 0;
    let home_dir = env::var("HOME")?;
    let record_fpath_full = format!("{}{}", home_dir, RECORD_FPATH);
    let round_or_rounds = if args.rounds == 1 { "round" } else { "rounds" };
    let is_are = if args.rounds == 1 { "is" } else { "are" };

    // load history here
    if !std::path::Path::new(&record_fpath_full).exists() {
        persist_history(record_fpath_full.as_str(), ResultHistory::init())?;
    }
    let hist = load_history(record_fpath_full.as_str())?; 

    let welcome_name_msg = format!("Welcome {}!", args.name);
    let round_info_msg = format!("Total {} of this game {} {}", round_or_rounds, is_are, args.rounds);

    println!("{}", ansi_term::Color::Blue.bold().paint("(=^_^=) ROMAN NUMBER TRANSLATION GAME!! (=^_^=) by Tanuki & Neko"));
    println!("{} {}", ansi_term::Color::Green.paint(welcome_name_msg), ansi_term::Color::Cyan.paint(round_info_msg));
    println!("{}", hist.to_string());
    println!("{}", ansi_term::Color::Cyan.paint("To quit the game, enter ctrl + c."));

    while counter <= args.rounds {
        print!("[ROUND {} / {}] ", counter, args.rounds);
        let (rez, ans) = single_flow_with_input()?;
        if rez {
            correct += 1;
            println!("CORRECT!");
        } else {
            incorrect += 1;
            println!("Incorrect .. correct answer is {}", &ans);
        }
        counter += 1;
        if counter <= args.rounds {
            println!("[PLAYER {}] Results so far - total:{}, correct:{}, incorrect:{}", args.name, counter, correct, incorrect);
        } else {
            let final_score_msg = format!("[PLAYER {}] Final Score - total:{}, correct:{}, incorrect:{}", args.name, counter - 1, correct, incorrect);
            println!("{}", ansi_term::Color::Green.bold().paint(final_score_msg));

            // persist the record to history here
            let new_hist = ResultHistory {
                total_correct: hist.total_correct + correct,
                total_incorrect: hist.total_incorrect + incorrect,
                total_rounds: hist.total_rounds + (counter - 1) as u32,
                last_update: chrono::Local::now(), 
            };
            persist_history(&record_fpath_full.as_str(), new_hist)?;
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    #[clap(long, short, default_value_t = 1, value_parser = validate_round_range)]
    rounds:u16,    
    #[clap(long, short, default_value_t = String::from("guest"))]
    name:String,
}

fn validate_round_range(round_input:&str) -> Result<u16, String> {
    let num:u16= round_input.parse()
    .map_err(|_|{ format!("{} is not within a valid range. Valid range is from 1 to up to 1000", round_input)})?;
    if ROUND_RANGE.contains(&num) {
        Ok(num as u16)
    } else {
        Err(format!("Round {} is not in the range 1 - 1000", round_input))
    }
    
}
