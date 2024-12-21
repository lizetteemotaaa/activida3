use rocket::form::Form;
use rocket::response::Redirect;
use rocket::{State, serde::json::Json};
use diesel::prelude::*;
use diesel::result::Error;
use argon2::{self, Config};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::models::{NewUsuario, Usuario};
use crate::DbConn;

#[post("/registro", data = "<form>")]
pub async fn registro(form: Form<NewUsuario<'_>>, conn: DbConn) -> Result<Redirect, String> {
    let hash = hash_password(form.contraseña);
    let new_user = NewUsuario {
        nombre: form.nombre,
        email: form.email,
        contraseña: &hash,
        rol_id: 1, // Suponiendo que el rol predeterminado es '1'
    };
    
    // Insertar en la base de datos
    diesel::insert_into(usuarios::table)
        .values(&new_user)
        .execute(&*conn)
        .map_err(|e| e.to_string())?;

    Ok(Redirect::to("/login"))
}

fn hash_password(password: &str) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), b"some_salt", &config).unwrap()
}
