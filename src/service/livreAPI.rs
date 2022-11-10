use actix_web::{post,delete, web, HttpResponse, Responder};
use std::collections::HashMap;
use serde_json::Value;

struct Livre 
{
    id : u32,
    titre : String,
    description : String,
    auteur : String
}

#[post("/")]
pub async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[delete("/{id}")]
pub async fn delete(_id: web::Path<u32>) -> impl Responder {
    let mut data = HashMap::new();
    data.insert("1","Ceci est un premier text");
    data.insert("2","Ceci est une deuxieme text");
    data.insert("3","Ceci est un troisieme text");

    if data.contains_key(&*_id.to_string()) {
        println!("{}","trouvé");
        data.remove(&*_id.to_string());
        let serialized_data = serde_json::to_string(&data).unwrap();
        HttpResponse::Ok().body(serialized_data)
    }else {
        println!("{}","pas trouvé");
        HttpResponse::Ok().body("Aucun id trouvé")
    }
}

#[post("/{id}")]
pub async fn edit(_id: web::Path<u32>,req_body: String) -> impl Responder {
    let mut data = HashMap::new();
    data.insert("1","Ceci est un premier text");
    data.insert("2","Ceci est une deuxieme text");
    data.insert("3","Ceci est un troisieme text");


    let mut object: Value = serde_json::from_str(&req_body).unwrap();
    let s: String = object.get_mut("text").expect("REASON").to_string();
    let s_slice: &str = &s; 

    if data.contains_key(&*_id.to_string()) {
        println!("{}","trouvé");
        data.entry("1").and_modify(|k| *k = s_slice);
        let serialized_data = serde_json::to_string(&data).unwrap();
        HttpResponse::Ok().body(serialized_data)
    }else {
        println!("{}","pas trouvé");
        HttpResponse::Ok().body("Aucun id trouvé")
    }
}