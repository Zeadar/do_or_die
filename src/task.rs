use chrono::{offset, prelude::*};
use colored::Colorize;
use core::fmt;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SaveData {
    description: String,
    due: String,
}

impl SaveData {
    pub fn to_task(&self) -> Task {
        return Task::new(
            self.description.clone(),
            NaiveDate::from_str(&self.due[..]).unwrap(),
        );
    }
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub description: String,
    pub due: NaiveDate,
}

const DESC_WIDTH: usize = 50;

impl Task {
    pub fn new(description: String, due: NaiveDate) -> Task {
        Task { description, due }
    }

    pub fn header() -> String {
        format!("{:<DESC_WIDTH$}{:<10}", "Description", "Date")
            .yellow()
            .on_bright_black()
            .to_string()
    }

    pub fn to_savedata(&self) -> SaveData {
        SaveData {
            description: self.description.clone(),
            due: self.due.to_string(),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let today = offset::Local::now().date_naive();
        let mut description = UnicodeSegmentation::graphemes(&self.description[..], true)
            .take(DESC_WIDTH)
            .collect::<String>();
        let due = self.due.to_string().yellow().on_bright_black().to_string();

        if today > self.due {
            description = format!("{:<DESC_WIDTH$}", description.red().on_black());
        } else if today < self.due {
            description = format!("{:<DESC_WIDTH$}", description.green().on_black());
        } else {
            description = format!("{:<DESC_WIDTH$}", description.blue().on_black());
        }

        write!(f, "{}{}", description, due,)
    }
}
