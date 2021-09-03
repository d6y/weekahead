use chrono::prelude::*;
use std::fs::File;
use std::io::Error;
use std::io::Write;

mod journal;
use journal::Journal;

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
struct Options {
    /// How many weeks forward to skip.
    /// Hint: to pass a negative number use the "-d=-1" format rather that the "-d -1" stype.
    #[structopt(short, long, default_value = "0")]
    skip: i8,

    /// Don't write the file.
    #[structopt(short, long)]
    dry_run: bool,
}

fn main() -> Result<(), Error> {
    let options = Options::from_args();

    let journal = Journal::from_monday(Utc::today(), options.skip);
    if options.dry_run {
        println!("{:?}", journal);
    }

    let days: Vec<Date<Utc>> = journal.days();

    if !options.dry_run {
        let mut file = File::create(format!("{}.md", journal.title))?;
        writeln!(&file, "# {}", journal.title)?;

        for day in days {
            layout(&mut file, &day)?;
        }
    }

    Ok(())
}

fn layout(file: &mut File, day: &Date<Utc>) -> Result<(), Error> {
    let day_heading = day.format("%d %b %Y (%a)");
    let tags = day.format("#Journal/OnThisDay/%m/%d# #Journal/%Y/%m#");
    write!(file, "## {}\n\n{}\n- - - -\n", day_heading, tags)
}
