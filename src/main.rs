use std::error::Error;
use thirtyfour::prelude::*;
use tokio::{io::AsyncWriteExt, fs::File, time::*};
use serde::Serialize;


#[derive(Serialize)]
struct Product{
    name : String,
    price : String,
}

impl Product{
    fn default_new() ->Self {
        Product { 
            name: String::from(""), 
            price: String::from("") 
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Manipulando o browser
    let caps = DesiredCapabilities::firefox();
    // local host do geckodriver
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    // Indo para o mercado livre
    driver.goto("https://www.mercadolivre.com.br/").await?;
    // Encontrando a área de busca
    let elem_form = driver.find(By::ClassName("nav-search")).await?;
    // Encontrando dentro da área de busca a área de input
    let element_key = elem_form.find(By::ClassName("nav-search-input")).await?;
    // Escrevendo na área de input "notebooks"
    element_key.send_keys("notebooks").await?;
    // Apertando o butão de confirmação de pesquisa
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;
    let mut vec_products : Vec<Product> = Vec::new();
    driver.query(By::Css(".ui-search-layout")).first().await?;
    let element_cookie = driver.find(By::ClassName("cookie-consent-banner-opt-out__actions")).await?;
    let element_cookie_button = element_cookie.find(By::Css("button[class='cookie-consent-banner-opt-out__action cookie-consent-banner-opt-out__action--primary cookie-consent-banner-opt-out__action--key-accept']"))
        .await?;
    element_cookie_button.click().await?;
    // pegando os nomes dos notebooks
    loop{
        // verificando o carregamento do site
        driver.query(By::Css(".ui-search-layout")).first().await?;
        
        // encontrando o card dos produtos
        let notebook_cards = driver.find_all(By::ClassName("poly-card__content")).await?;
        for card in notebook_cards{
            let mut  product = Product::default_new();
            // pegando o preço dentro do card
            let price =  card.find(By::ClassName("andes-money-amount__fraction")).await?;
            // pegando o nome do produto dentro do card
            let name = card.find(By::ClassName("poly-component__title-wrapper")).await?;
            product.name = name.text().await?;
            product.price = price.text().await?;
            vec_products.push(product);
        }
        let form_button = match  driver.find(By::Css("li[class='andes-pagination__button andes-pagination__button--next']")).await{
            Ok(element) => element,
            Err(_) => break,
        };
        form_button.scroll_into_view().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let form_next_button = driver.find(By::ClassName("andes-pagination__button.andes-pagination__button--next")).await?;
        let button = form_next_button.find(By::Css("a[class='andes-pagination__link']")).await?;
        button.click().await?;

        // mudar o uso do sleep no futuro !!! 
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    let mut file  = File::create("products.json").await?;
    let json = serde_json::to_string_pretty(&vec_products)?;
    file.write_all(json.as_bytes()).await?;

    // Fechando o navegador
    driver.quit().await?;
    Ok(())
}