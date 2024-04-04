use raur::Raur;
use colored::Colorize;
use clap::Parser;

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
            println!("Please choose from the list of packages to install.");

            let mut index: i32 = 1;

            for (index, pkg) in pkgs.iter().enumerate() {
                println!("[{}] {:<30} {}", (index as usize) + 1, pkg.name, pkg.version.green());
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