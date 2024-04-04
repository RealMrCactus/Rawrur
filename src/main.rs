use raur::{Package, Raur};
use colored::Colorize;
use clap::Parser;
use std::{collections::HashMap, io::{stdout, Write}, ops::Not};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    sync: bool,
    
    query: String,
}
    
async fn sync() -> std::result::Result<(), std::io::Error> {
    Ok(())
}

async fn handlesearch(query: &str, sync:bool) -> std::result::Result<(), raur::Error> {
    let raur = raur::Handle::new();

    // Use `search` to search using keywords (multiple strategies available)
    let pkgs = raur.search(query).await?;
    assert!(pkgs.len() > 1);
    
    let mut many: i32 = 0;
    for pkg in &pkgs {
        if ! sync {
            println!("{:<30} {}", pkg.name, pkg.version.green());
        } else {
            many = many + 1;
        }
    }

    if sync {
        println!("{} package(s) found", many);
        if many > 1 {
            // map packages to a number map
            let mut pkg_map: HashMap<usize, &Package> = HashMap::new();

            for (index, pkg) in pkgs.iter().enumerate() {
                println!("[{}] {:<30} {}", ((index as usize) + 1).to_string().red(), pkg.name, pkg.version.green());
                pkg_map.insert(index + 1, pkg);
            }
            
            // prompt the user.
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
            print!("\nDo you want to install {}? [Y/n] ", selected.unwrap().name);
            std::io::stdout().flush().unwrap();

            // get their answer
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap().to_string();
            println!("{}", input);
            if input.to_lowercase().trim() == "y" || input.trim().is_empty() {
                println!("Installing {}...", selected.unwrap().name);
            }
        }
    }
    Ok(())
}

async fn search(query: &str, sync: bool) {
    match handlesearch(query, sync).await {   
        Ok(_) => println!("\n"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    search(&args.query, args.sync).await;
}