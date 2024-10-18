mod args;
mod util;

use raur::{Package, Raur};
use colored::Colorize;
use clap::Parser;
use std::{collections::HashMap, io::{stdout, Write}, ops::Not};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

// Use util/mod.rs for conf messages soon.
struct Args {
    #[clap(short, long, default_value = "false")]
    sync: bool,
    #[clap(short, long, default_value = "false")]
    noconfirm: bool,
    #[clap(short, long, default_value = "")]
    query: String,
    #[clap(short, long, default_value = "")]
    remove: String,
}

async fn handlesearch(query: &str, sync: bool, noconf: bool) -> std::result::Result<(), raur::Error> {
    let raur = raur::Handle::new();

    // Use `search` to search using keywords.
    let pkgs = raur.search(query).await?;
    assert!(pkgs.len() > 1);
    
    let mut many: i32 = 0;
    for pkg in &pkgs {
        if ! sync { // Does this even work????
            println!("{:<30} {}", pkg.name, pkg.version.green());
        } else {
            // Enumerate over how many packages are shown with the query provided.
            many += 1;
        }
    }

    if sync {
        println!("{} package(s) found", many);
        if many > 1 {
            // map packages to a number map
            let mut pkg_map: HashMap<usize, &Package> = HashMap::new();

            // List the packages found
            for (index, pkg) in pkgs.iter().enumerate() {
                println!("[{}] {:<30} {}", ((index as usize) + 1).to_string().red(), pkg.name, pkg.version.green());
                pkg_map.insert(index + 1, pkg);
            }
            
            /* prompt the user.

            USE CROSSTERM

            */
            
            println!("\n\nPlease select a package from the list above to install");
            print!("> ");
            std::io::stdout().flush().unwrap();
            
            // get input from the user.
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap().to_string();
            
            // check if the input is valid
            if input.trim().is_empty() || input.trim().parse::<usize>().unwrap() < 1 {
                eprintln!("{}", "\nInvalid input".red());
                return Ok(());
            }
            
            // get the package the user asked for.
            let selected = pkg_map.get(&input.trim().parse::<usize>().unwrap());

            // prompt the user to install
            let ans = util::yn_prompt_install(format!("{:?}", selected.unwrap().name), noconf);
            
            print!("{}", ans);

            // install pkg

        } else if many == 1 {
            let selected = pkgs.first();

            // prompt the user to install
            let ans = util::yn_prompt_install(format!("{:?}", selected), noconf);
            
            print!("{}", ans);
        }
    }

    Ok(())
}

async fn search(query: &str, sync: bool, noconf: bool) {
    if query.trim().is_empty() {
        eprintln!("{}", "No query recived.".red());
    }

    match handlesearch(query, sync, noconf).await {   
        Ok(_) => println!("\n"),
        Err(e) => eprintln!("Error: {}", e.to_string().red()),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // check for args.query
    if !args.query.is_empty() {
        search(&args.query, args.sync, args.noconfirm).await;
    }
}
