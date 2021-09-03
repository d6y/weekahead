use std::ops::Add;

use chrono::{prelude::*, Duration};

#[derive(Debug)]
pub struct Journal {
    start: Date<Utc>,
    pub title: String,
}

impl Journal {
    pub fn from_monday(now: Date<Utc>, weeks_forward: i8) -> Journal {
        let skip = Duration::weeks(weeks_forward as i64);
        let start = find_next_monday(now).add(skip);
        let title = start.format("Week %d %b %Y").to_string();
        Journal { start, title }
    }

    pub fn days(&self) -> Vec<Date<Utc>> {
        let mut cursor = self.start;
        let mut days = Vec::with_capacity(7);

        (1..=7).for_each(|_| {
            days.push(cursor);
            cursor = cursor.succ();
        });

        days
    }
}

fn find_next_monday(from: Date<Utc>) -> Date<Utc> {
    if from.weekday() == Weekday::Mon {
        from
    } else {
        find_next_monday(from.succ())
    }
}
