use reqwest::*;
use raur::{Package, Raur};
use colored::Colorize;
use clap::Parser;
use std::{collections::HashMap, io::{stdout, Write}, ops::Not};


async fn pull(url: &str) -> std::result::Result<(), std::io::Error> {
    
    

    Ok(())
}
    
pub async fn sync(pkg: &str) -> std::result::Result<(), std::io::Error> {
    // process for downloading from the AUR

    pull(pkg).await?;
    Ok(())
}