use crate::models::Product;
use std::error::Error;
use thirtyfour::prelude::*;


pub async fn fill_products_vec(driver : &WebDriver) -> Result<Vec<Product>, Box<dyn Error + Send + Sync>>{
    let product_card = driver.find_all(By::ClassName("poly-card__content")).await?;
    let mut vec_products : Vec<Product> = Vec::new();
    for card in product_card{
        let mut product = Product::default_new();
        // pegando o preço dentro do card
        let price =  card.find(By::ClassName("andes-money-amount__fraction")).await?;
        // pegando o nome do produto dentro do card
        let name = card.find(By::ClassName("poly-component__title-wrapper")).await?;
        product.name = name.text().await?;
        product.price = price.text().await?;
        vec_products.push(product);
    }
    Ok(vec_products)
}