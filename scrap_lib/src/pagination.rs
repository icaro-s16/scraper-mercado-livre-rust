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
    button.click().await?;

    // mudar o uso do sleep no futuro !!! 
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok(Some(()))
}