use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rol {
    pub id: i32,
    pub nombre: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Permiso {
    pub id: i32,
    pub nombre: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub contraseña: String,
    pub rol_id: i32,
    pub creado_en: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "usuarios"]
pub struct NewUsuario<'a> {
    pub nombre: &'a str,
    pub email: &'a str,
    pub contraseña: &'a str,
    pub rol_id: i32,
}
#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket::serde::{json::Json, Deserialize};
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![registro, login])
        .attach(Template::fairing())
}

