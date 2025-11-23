use serde::Serialize;


#[derive(Serialize)]
pub struct Product{
    pub name : String,
    pub price : String,
}

impl Product{
    pub fn default_new() ->Self {
        Product { 
            name: String::from(""), 
            price: String::from("") 
        }
    }
}
