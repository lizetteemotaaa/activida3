use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::form::Form;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::models::{Usuario};
use crate::DbConn;

#[post("/login", data = "<form>")]
pub async fn login(form: Form<Login<'_>>, conn: DbConn) -> Result<Json<String>, String> {
    // Verificar las credenciales
    let user = usuarios::table
        .filter(usuarios::email.eq(form.email))
        .first::<Usuario>(&*conn)
        .map_err(|_| "Usuario o contraseña incorrectos".to_string())?;

    if !verify_password(form.contraseña, &user.contraseña) {
        return Err("Usuario o contraseña incorrectos".to_string());
    }

    let my_claims = Claims { 
        sub: user.id.to_string(), 
        exp: chrono::Utc::now().timestamp() as usize + 60 * 60 * 24, // 1 día de expiración
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret_key".as_ref()),
    ).map_err(|e| e.to_string())?;

    Ok(Json(token))
}

fn verify_password(plain_password: &str, hashed_password: &str) -> bool {
    argon2::verify_encoded(hashed_password, plain_password.as_bytes()).unwrap_or(false)
}
