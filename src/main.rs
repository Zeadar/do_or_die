use std::io::{Read, Write};

use chrono::{offset, prelude::*};
mod console;
use crate::console::*;
mod accounter;
mod task;
use task::Task;

fn main() {
    let mut current_view = offset::Local::now().date_naive();
    let mut coordinator = accounter::Coordinator::new();
    let path = std::path::Path::new("save.json");

    if path.exists() {
        let mut file = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        coordinator.deserialzie(content.as_str());
        green_line(format!("Loaded {:?}!", path));
    } else {
        red_line(format!("Could not find {:?}", path));
        green_line(format!("Will create {:?} on save, don't worry!", path));
    }

    println!("██████   ██████       ██████  ██████      ██████  ██ ███████ ██");
    println!("██   ██ ██    ██     ██    ██ ██   ██     ██   ██ ██ ██      ██");
    println!("██   ██ ██    ██     ██    ██ ██████      ██   ██ ██ █████   ██");
    println!("██   ██ ██    ██     ██    ██ ██   ██     ██   ██ ██ ██");
    println!("██████   ██████       ██████  ██   ██     ██████  ██ ███████ ██");

    print!("{}", "\n".repeat(4));

    main_menu(&mut coordinator, &mut current_view);

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(coordinator.serialize().as_bytes()).unwrap();
}

fn main_menu(coordinator: &mut accounter::Coordinator, current_view: &mut NaiveDate) {
    loop {
        green_line(format!("Selected date {current_view}"));
        println!("Enter [_] for which option you select");
        println!("[L]ist tasks");
        println!("[A]dd tasks");
        println!("[C]hange view date");
        println!("[Q]uit");

        let input = prompt_string("[L] [A] [C] [Q]");

        match &input[..] {
            "L" => list_tasks(coordinator),
            "A" => add_task(coordinator, current_view),
            "C" => change_view(current_view),
            "Q" => break,
            _ => red_line(format!("Unrecognised command: \"{}\"", input)),
        }
    }
}

fn list_tasks(coordinator: &accounter::Coordinator) {
    println!("{}", Task::header());
    for task in coordinator.get_tasks() {
        println!("{task}");
    }
}

fn add_task(coordinator: &mut accounter::Coordinator, view: &NaiveDate) {
    let desc = prompt_string("Enter description [string]");
    let task = Task::new(desc, *view);

    green_line(format!("Added task {task}"));
    coordinator.add_task(task);
}

fn change_view(current_view: &mut NaiveDate) {
    loop {
        green_line(format!("{current_view}"));
        println!("Enter new [Y]ear");
        println!("Enter new [M]onth");
        println!("Enter new [D]ate");
        println!("[Q]uit");

        let input = prompt_string("[Y] [M] [Q]");

        match &input[..] {
            "M" => {
                match current_view.with_month(promt_u32("Enter month [M]/[MM]")) {
                    Some(view) => {
                        *current_view = view;
                        green_line(format!("Changed viewing month to {}", view.month0() + 1));
                        break;
                    }
                    None => {
                        red_line(format!("Could not use {input} as month"));
                        continue;
                    }
                };
            }
            "Y" => {
                match current_view.with_year(promt_i32("Enter year [yyyy]")) {
                    Some(view) => {
                        *current_view = view;
                        green_line(format!("Changed viewing year to {}", view.year_ce().1));
                        break;
                    }
                    None => {
                        red_line(format!("Could not use {input} as year"));
                        continue;
                    }
                };
            }
            "D" => {
                match current_view.with_day(promt_u32("Enter day [dd]")) {
                    Some(view) => {
                        *current_view = view;
                        green_line(format!("Changed viewing day to {}", view.day0() + 1));
                        break;
                    }
                    None => {
                        red_line(format!("Could not use {input} as day"));
                        continue;
                    }
                };
            }
            "Q" => break,
            _ => red_line(format!("Unrecognised command: \"{}\"", input)),
        }
    }
}
