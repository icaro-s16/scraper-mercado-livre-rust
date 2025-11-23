use tokio::{io::AsyncWriteExt, fs::File};
use crate::models::Product;
use serde_json;
use std::error::Error;


pub async fn file_product_json(products_vec : &Vec<Product>, product : &str) -> Result<(), Box<dyn Error + Send + Sync>>{
    let mut file  = File::create(format!("{}.json", product)).await?;
    let json = serde_json::to_string_pretty(&products_vec)?;
    file.write_all(json.as_bytes()).await?;
    Ok(())
}
