use chrono::{offset, prelude::*};
use std::io::{Read, Write};
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
    file.write_all(format!("{}\n", coordinator.serialize()).as_bytes())
        .unwrap();
}

fn main_menu(coordinator: &mut accounter::Coordinator, current_view: &mut NaiveDate) {
    loop {
        green_line(format!("Selected date {current_view}"));
        println!("Enter [_] for which option you select");
        println!("[L]ist tasks");
        println!("[A]dd tasks");
        println!("[R]emove taks");
        println!("[C]hange view date");
        println!("[Q]uit");

        let input = prompt_string("[L] [A] [R] [C] [Q]");

        match &input[..] {
            "L" => list_tasks(coordinator),
            "A" => add_task(coordinator, current_view),
            "R" => remove_tasks(coordinator),
            "C" => change_view(current_view),
            "Q" => break,
            _ => red_line(format!("Unrecognised command: \"{}\"", input)),
        }
    }
}

fn remove_tasks(coordinator: &mut accounter::Coordinator) {
    let position: usize;

    {
        // Apparently coordinator gets borrowed as immutable next line
        // and thus needs to be given back before it is possible
        // to call any &mut methods...
        let tasks = coordinator.get_tasks();

        if tasks.is_empty() {
            red_line("No tasks to remove".to_string());
            return;
        }

        for (i, task) in tasks.iter().enumerate() {
            println!("{:>3} {}", i, task);
        }

        let index: usize = (console::promt_u32(format!("[0..{}]", tasks.len()).as_str()))
            .try_into()
            .unwrap();

        let candidate = match tasks.get(index) {
            Some(some) => some,
            None => {
                console::red_line(format!("{} is out of range", index));
                return;
            }
        };

        position = coordinator.get_position(&candidate).unwrap();
    }

    coordinator.del_task(position);
}

fn list_tasks(coordinator: &accounter::Coordinator) {
    println!("{}", Task::header());
    for task in coordinator.get_tasks() {
        println!("{task}");
    }
}

fn add_task(coordinator: &mut accounter::Coordinator, view: &NaiveDate) {
    let mut desc = prompt_string("Enter description [string]");
    if desc.is_empty() {
        red_line("Empty string".to_string());
        return;
    }

    desc = format!("{}{}", &desc[0..1], &desc[1..].to_lowercase());
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
