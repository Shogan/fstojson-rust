use std::{
    fs::{self},
    io::{Error, ErrorKind},
    path::Path,
};

use clap::{App, Arg};
use indextree::{Arena, NodeId};
use serde::{Deserialize, Serialize};
use serde_indextree::Node;
use serde_json::{to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PathNode {
    name: String,
    relative_path: String,
    absolute_path: String,
    node_type: String,
}

fn main() -> Result<(), Error> {
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new("fstojson")
        .version(version)
        .author("Sean Duffy - https://github.com/shogan")
        .about("Traverses a target filesystem directory & outputs the collected hierarchy to JSON")
        .arg(
            Arg::with_name("path")
                .short("t")
                .long("traverse")
                .value_name("PATH")
                .help("The directory path to traverse")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("p")
                .short("p")
                .multiple(false)
                .help("Enable pretty JSON print output"),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .multiple(false)
                .help("Traverse hierarchy recursively"),
        )
        .get_matches();

    let pretty_print;
    let recursive;

    match matches.occurrences_of("r") {
        0 => recursive = false,
        1 | _ => recursive = true,
    }

    match matches.occurrences_of("p") {
        0 => pretty_print = false,
        1 | _ => pretty_print = true,
    }

    if matches.value_of("path") != None {
        let directory_path = matches.value_of("path").unwrap();
        let root_path = Path::new(directory_path);

        if root_path.is_dir() {
            let mut absolute_path = std::env::current_dir()?;
            absolute_path.push(root_path);

            let arena = &mut Arena::new();

            let root_node = arena.new_node(PathNode {
                name: directory_path.to_string(),
                node_type: "Directory".to_string(),
                relative_path: directory_path.to_string(),
                absolute_path: absolute_path.display().to_string(),
            });

            traverse(directory_path, arena, root_node, recursive)?;
            if pretty_print {
                println!(
                    "{}",
                    to_string_pretty(&Node::new(root_node, arena)).unwrap()
                );
            } else {
                println!("{}", to_string(&Node::new(root_node, arena)).unwrap());
            }

            Ok(())
        } else {
            eprintln!("Invalid directory.");
            Result::Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid directory: ".to_string() + directory_path,
            ))
        }
    } else {
        Result::Err(Error::new(
            ErrorKind::InvalidInput,
            "No path provided.".to_string(),
        ))
    }
}

fn traverse(
    path: &str,
    arena: &mut Arena<PathNode>,
    parent: NodeId,
    recursive: bool,
) -> Result<(), Error> {
    let dir_listing = get_directory_listing(path);
    for entry in dir_listing {
        let temp_path = Path::new(entry.as_str());
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(temp_path);

        if temp_path.is_dir() {
            let dir_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                node_type: String::from("Directory"),
            });

            parent.append(dir_object, arena);

            if recursive {
                traverse(entry.as_str(), arena, dir_object, recursive)?;
            }
        } else {
            let file_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                node_type: String::from("File"),
            });

            parent.append(file_object, arena);
        }
    }

    Ok(())
}

fn get_directory_listing(directory_path: &str) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap()
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect()
}