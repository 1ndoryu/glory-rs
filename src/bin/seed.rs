/* [253A-16] Script de seed para generar datos de prueba realistas.
 * Crea un usuario demo (demo@restaurante.com / demo1234) y
 * inserta ventas, gastos y reservas del mes actual y los 2 anteriores.
 * Uso: cargo run --bin seed */

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL requerido");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("No se pudo conectar a la BD");

    println!("Conectado a la base de datos.");

    /* Paso 1: crear usuario demo */
    let email = "demo@restaurante.com";
    let password = "demo1234";

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error al hashear password")
        .to_string();

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, password_hash, nombre) VALUES ($1, $2, $3) \
         ON CONFLICT (email) DO UPDATE SET password_hash = $2 \
         RETURNING id",
    )
    .bind(email)
    .bind(&hash)
    .bind("Demo Admin")
    .fetch_one(&pool)
    .await
    .expect("Error al crear usuario demo");

    println!("Usuario demo creado: {email} (id: {user_id})");

    /* Paso 2: generar datos para los ultimos 3 meses */
    let hoy = Local::now().date_naive();
    let inicio = primer_dia_mes(hoy) - Duration::days(60);

    seed_ventas(&pool, user_id, inicio, hoy).await;
    seed_gastos(&pool, user_id, inicio, hoy).await;
    seed_reservas(&pool, user_id, hoy).await;

    println!("Seed completado exitosamente.");
}

fn primer_dia_mes(d: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d)
}

async fn seed_ventas(pool: &PgPool, user_id: Uuid, desde: NaiveDate, hasta: NaiveDate) {
    let turnos = ["manana", "mediodia", "noche"];
    let canales = ["comedor", "barra", "terraza", "delivery", "just_eat", "eventos"];
    let metodos = ["efectivo", "tarjeta", "transferencia"];
    let descripciones = [
        "Menu del dia", "Cena grupo", "Pedido delivery", "Evento privado",
        "Comida rapida barra", "Menu ejecutivo", "Cena romantica",
        "Brunch dominical", "Tapas variadas", "Comida terraza",
    ];

    let mut fecha = desde;
    let mut count: u32 = 0;
    let mut idx: usize = 0;

    while fecha <= hasta {
        /* 2-5 ventas por dia */
        let num_ventas = 2 + (idx % 4);
        for i in 0..num_ventas {
            let base = Decimal::from_str(&format!("{}.{:02}", 45 + (idx * 17 + i * 23) % 300, (idx * 7 + i * 13) % 100))
                .unwrap_or_else(|_| Decimal::from(100));
            let iva = (base * Decimal::from_str("0.10").unwrap_or_default())
                .round_dp(2);
            let comensales = 1 + (idx + i) % 8;

            sqlx::query(
                "INSERT INTO ventas (user_id, fecha, comensales, descripcion, iva_porcentaje, turno, canal, metodo_pago, importe_base, importe_iva) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
            )
            .bind(user_id)
            .bind(fecha)
            .bind(i32::try_from(comensales).unwrap_or(2))
            .bind(descripciones[(idx + i) % descripciones.len()])
            .bind(Decimal::from(10))
            .bind(turnos[(idx + i) % turnos.len()])
            .bind(canales[(idx + i) % canales.len()])
            .bind(metodos[(idx + i) % metodos.len()])
            .bind(base)
            .bind(iva)
            .execute(pool)
            .await
            .expect("Error al insertar venta");

            count += 1;
        }
        fecha += Duration::days(1);
        idx += 1;
    }
    println!("  {count} ventas insertadas.");
}

async fn seed_gastos(pool: &PgPool, user_id: Uuid, desde: NaiveDate, hasta: NaiveDate) {
    /* Obtener IDs de categorias existentes */
    let categorias: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM categorias_gasto ORDER BY nombre")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let tipos = ["factura", "albaran", "ticket"];
    let metodos = ["efectivo", "tarjeta", "transferencia"];
    let proveedores = [
        "Distribuciones Lopez", "Carnes Ibericas S.L.", "Pescados del Norte",
        "Cervecera Nacional", "Verduras Eco", "Limpieza Total",
        "Endesa Energia", "Vodafone Business", "Reparaciones Martinez",
        "Papeleria Central",
    ];

    let mut fecha = desde;
    let mut count: u32 = 0;
    let mut idx: usize = 0;

    while fecha <= hasta {
        /* 1-3 gastos por dia */
        let num_gastos = 1 + (idx % 3);
        for i in 0..num_gastos {
            let base = Decimal::from_str(&format!("{}.{:02}", 20 + (idx * 13 + i * 19) % 500, (idx * 3 + i * 7) % 100))
                .unwrap_or_else(|_| Decimal::from(50));
            let iva = (base * Decimal::from_str("0.21").unwrap_or_default())
                .round_dp(2);

            let cat_id = if categorias.is_empty() {
                None
            } else {
                Some(categorias[(idx + i) % categorias.len()])
            };

            sqlx::query(
                "INSERT INTO gastos (user_id, fecha, proveedor, categoria_id, tipo_documento, metodo_pago, numero_documento, recurrente, importe_base, importe_iva) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
            )
            .bind(user_id)
            .bind(fecha)
            .bind(proveedores[(idx + i) % proveedores.len()])
            .bind(cat_id)
            .bind(tipos[(idx + i) % tipos.len()])
            .bind(metodos[(idx + i) % metodos.len()])
            .bind(format!("DOC-{:04}", count + 1))
            .bind((idx + i).is_multiple_of(5))
            .bind(base)
            .bind(iva)
            .execute(pool)
            .await
            .expect("Error al insertar gasto");

            count += 1;
        }
        fecha += Duration::days(1);
        idx += 1;
    }
    println!("  {count} gastos insertados.");
}

async fn seed_reservas(pool: &PgPool, user_id: Uuid, hoy: NaiveDate) {
    let nombres = [
        "Maria Garcia", "Carlos Rodriguez", "Ana Martinez", "Pedro Sanchez",
        "Laura Fernandez", "Javier Lopez", "Carmen Ruiz", "Miguel Torres",
        "Lucia Moreno", "David Jimenez", "Sofia Navarro", "Pablo Romero",
        "Elena Diaz", "Sergio Munoz", "Teresa Alonso", "Raul Gutierrez",
    ];
    let estados = ["pendiente", "confirmada", "confirmada", "cancelada"];
    let horas = [
        NaiveTime::from_hms_opt(13, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(13, 30, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(14, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(20, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(20, 30, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(21, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(21, 30, 0).unwrap_or_default(),
    ];

    let mut count: u32 = 0;

    /* Reservas para hoy y los proximos 14 dias */
    for dia in 0_u32..15 {
        let fecha = hoy + Duration::days(i64::from(dia));
        let num_reservas = 3 + (dia as usize % 5);

        for i in 0..num_reservas {
            let idx = (dia as usize * 7 + i) % nombres.len();
            let personas = 2 + i % 6;
            let telefono = format!("6{:08}", 10_000_000 + count * 1234 + dia * 100);

            sqlx::query(
                "INSERT INTO reservas (user_id, fecha, hora, nombre_cliente, num_personas, estado, notas, telefono) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(user_id)
            .bind(fecha)
            .bind(horas[i % horas.len()])
            .bind(nombres[idx])
            .bind(i32::try_from(personas).unwrap_or(2))
            .bind(if dia == 0 && i < 2 { "confirmada" } else { estados[i % estados.len()] })
            .bind(if i % 3 == 0 { "Mesa junto a ventana" } else { "" })
            .bind(&telefono)
            .execute(pool)
            .await
            .expect("Error al insertar reserva");

            count += 1;
        }
    }
    println!("  {count} reservas insertadas.");
}
