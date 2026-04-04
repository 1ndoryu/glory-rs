/* [044A-34] Seed: crea usuario admin/admin para desarrollo local.
 * Ejecutar: cargo run --example seed_admin
 * Requiere DATABASE_URL en .env o como variable de entorno. */

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use argon2::password_hash::rand_core::OsRng;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida en .env");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    let email = "admin@admin.com";
    let password = "admin";

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error al hashear contraseña")
        .to_string();

    let id = Uuid::new_v4();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3) ON CONFLICT (email) DO UPDATE SET password_hash = $3"
    )
    .bind(id)
    .bind(email)
    .bind(&password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => println!("Usuario admin creado: email={email}, password={password}"),
        Err(e) => eprintln!("Error al crear usuario admin: {e}"),
    }
}
