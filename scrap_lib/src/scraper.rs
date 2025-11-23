use std::error::Error;
use thirtyfour::prelude::*;
use crate::pagination::pagination;
use crate::extractor::fill_products_vec;
use crate::storage::file_product_json;
use crate::models::{Product, GenericInput};



pub async fn scrap(product : &str, num : &GenericInput<'_>) -> Result<Option<()>, Box<dyn Error + Send + Sync>>{
    // Manipulando o browser
    let caps = DesiredCapabilities::firefox();
    // local host do geckodriver
    let driver = match WebDriver::new("http://localhost:4444", caps).await{
        Ok(drive) => drive,
        Err(_) => {
            return Ok(None);
        }
    };
    // Indo para o mercado livre
    driver.goto("https://www.mercadolivre.com.br/").await?;
    // Encontrando a área de busca
    let elem_form = driver.find(By::ClassName("nav-search")).await?;
    // Encontrando dentro da área de busca a área de input
    let element_key = elem_form.find(By::ClassName("nav-search-input")).await?;
    // Escrevendo na área de input o produto
    element_key.send_keys(product).await?;
    // Apertando o butão de confirmação de pesquisa
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;
    let mut vec_products : Vec<Product> = Vec::new();
    match num{    
        GenericInput::Text(_text) => {
            loop{
                // verificando o carregamento do site
                driver.query(By::Css(".ui-search-layout")).first().await?;
                // encontrando o card dos produtos
                let mut vec = fill_products_vec(&driver).await?;
                vec_products.append(&mut vec);

                match pagination(&driver).await{
                    Ok(form) => {
                        match form{
                            None => {
                                break;
                            },
                            Some(_) => (),
                        };
                    }
                    Err(erro) => {
                        eprintln!("Error: Failed to complete scraping operation. Reason: {}", erro.to_string());
                        return Err(erro);
                    },
                };
            }
        }
        GenericInput::Number(num) =>{ 
            for pages in 0 .. *num{
                // verificando o carregamento do site
                driver.query(By::Css(".ui-search-layout")).first().await?;
                // encontrando o card dos produtos
                let mut vec = fill_products_vec(&driver).await?;
                vec_products.append(&mut vec);

                match pagination(&driver).await{
                    Ok(form) => {
                        match form{
                            None => {
                                println!("Scraping finished on page {} because there are no more pages to scrape.", pages+1);
                                break;
                            },
                            Some(_) => (),
                        };
                    }
                    Err(erro) => {
                        eprintln!("Error: Failed to complete scraping operation. Reason: {}", erro.to_string());
                        return Err(erro);
                    },
                };
            }
        }
        _ => (),
    }
    match file_product_json(&vec_products, product).await{
        Err(erro) => {
            eprintln!("Error: Failed to complete scraping operation. Reason: {}", erro.to_string());
            return Err(erro);
        }
        _ => (),
    };
    // Fechando o navegador
    driver.quit().await?;
    Ok(Some(()))
}
