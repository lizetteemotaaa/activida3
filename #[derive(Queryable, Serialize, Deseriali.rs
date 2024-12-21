#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub hashed_password: String,
}

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