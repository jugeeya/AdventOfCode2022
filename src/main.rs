use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of the calendar to run
   #[arg(short, long)]
   day: u32,

   /// Part of problem to run
   #[arg(short, long)]
   part: u32,

   /// Input filepath
   #[arg(short, long)]
   filepath: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    
    match args.day {
        1 => match args.part {
            1 => day_1::part_1(&args.filepath)?,
            2 => day_1::part_2(&args.filepath)?,
            p => panic!("Unknown part {p}")
        },
        2 => match args.part {
            1 => day_2::part_1(&args.filepath)?,
            2 => day_2::part_2(&args.filepath)?,
            p => panic!("Unknown part {p}")
        },
        3 => match args.part {
            1 => day_3::part_1(&args.filepath)?,
            2 => day_3::part_2(&args.filepath)?,
            p => panic!("Unknown part {p}")
        },
        4 => match args.part {
            1 => day_4::part_1(&args.filepath)?,
            2 => day_4::part_2(&args.filepath)?,
            p => panic!("Unknown part {p}")
        },
        d => panic!("Unknown part {d}")
    }

    Ok(())
}
