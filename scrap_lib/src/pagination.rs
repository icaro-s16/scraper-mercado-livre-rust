use std::error::Error;
use thirtyfour::prelude::*;
use tokio::time::*;


pub async fn pagination(driver : &WebDriver)->Result<Option<()>, Box<dyn Error + Send + Sync>>{
    let form_button = match  driver.find(By::Css("li[class='andes-pagination__button andes-pagination__button--next']")).await{
            Ok(element) => element,
            Err(_) => return Ok(None),
    };
    form_button.scroll_into_view().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let form_next_button = driver.find(By::ClassName("andes-pagination__button.andes-pagination__button--next")).await?;
    let button = form_next_button.find(By::Css("a[class='andes-pagination__link']")).await?;
    let product_card_atual = driver.find(By::ClassName("poly-card__content")).await?;
    
    button.click().await?;

    // espera até que o primeiro produto da página anterior estiver "morrido"
    product_card_atual.wait_until().stale().await?;
    Ok(Some(()))
}