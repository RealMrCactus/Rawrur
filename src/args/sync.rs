use reqwest::*;
use raur::{Package, Raur};
use std::{collections::HashMap, io::{stdout, Write}, ops::Not};


async fn pull(url: String) -> std::result::Result<(), std::io::Error> {
    
    

    Ok(())
}
    
pub async fn sync(pkg: Package) -> std::result::Result<(), std::io::Error> {
    // process for downloading from the AUR

    // pull(pkg).await?;
    Ok(())
}