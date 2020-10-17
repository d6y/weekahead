use chrono::prelude::*;
use std::fs::File;
use std::io::Error;
use std::io::Write;

fn main() -> Result<(), Error> {
    let first_day = monday(Utc::today());
    let days: Vec<Date<Utc>> = week(first_day);

    let title = first_day.format("Week %d %b %Y");

    let mut file = File::create(format!("{}.md", title))?;
    writeln!(&file, "# {}", title)?;

    for day in days {
        layout(&mut file, &day)?;
    }

    Ok(())
}

fn layout(file: &mut File, day: &Date<Utc>) -> Result<(), Error> {
    let day_heading = day.format("%d %b %Y (%a)");
    let tags = day.format("#Journal/OnThisDay/%m/%d# #Journal/%Y/%m#");
    write!(file, "## {}\n\n\n{}\n\n---\n\n", day_heading, tags)
}

fn monday(from: Date<Utc>) -> Date<Utc> {
    if from.weekday() == Weekday::Mon {
        from
    } else {
        monday(from.succ())
    }
}

fn week(from: Date<Utc>) -> Vec<Date<Utc>> {
    let mut cursor = from;
    let mut days = Vec::with_capacity(7);

    (1..=7).for_each(|_| {
        days.push(cursor);
        cursor = cursor.succ();
    });

    days
}
