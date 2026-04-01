/* [263A-12] Script de seed actualizado — genera datos de prueba realistas.
 * Crea usuario demo (demo@restaurante.com / demo1234) e inserta:
 * canales de reserva, clientes, ventas, gastos, reservas (con no-shows
 * y canal_id/cliente_id), y asigna etiquetas del sistema a clientes.
 * Idempotente: limpia datos del usuario demo antes de insertar.
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

    /* Crear/actualizar usuario demo */
    let email = "demo@restaurante.com";
    let password = "demo1234";

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error al hashear password")
        .to_string();

    let user_id: Uuid = sqlx::query_scalar!(
        "INSERT INTO users (email, password_hash, nombre) VALUES ($1, $2, $3) \
         ON CONFLICT (email) DO UPDATE SET password_hash = $2 \
         RETURNING id",
        email,
        &hash,
        "Demo Admin"
    )
    .fetch_one(&pool)
    .await
    .expect("Error al crear usuario demo");

    println!("Usuario demo: {email} (id: {user_id})");

    /* Limpiar datos anteriores del usuario demo (idempotente) */
    limpiar_datos(&pool, user_id).await;

    /* Generar datos de prueba */
    let hoy = Local::now().date_naive();
    let inicio = primer_dia_mes(hoy) - Duration::days(60);

    let canal_ids = seed_canales(&pool, user_id).await;
    let cliente_ids = seed_clientes(&pool, user_id).await;
    seed_ventas(&pool, user_id, inicio, hoy).await;
    seed_gastos(&pool, user_id, inicio, hoy).await;
    seed_reservas(&pool, user_id, hoy, &canal_ids, &cliente_ids).await;
    seed_etiquetas_clientes(&pool, user_id, &cliente_ids).await;
    seed_campanas(&pool, user_id).await;
    seed_plantillas_whatsapp(&pool, user_id).await;
    seed_reglas_recordatorio(&pool, user_id).await;

    println!("\nSeed completado exitosamente.");
}

fn primer_dia_mes(d: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap_or(d)
}

/* Limpia todos los datos del usuario demo respetando FKs */
async fn limpiar_datos(pool: &PgPool, user_id: Uuid) {
    let sentencias = [
        "DELETE FROM campana_destinatarios WHERE campana_id IN (SELECT id FROM campanas WHERE user_id = $1)",
        "DELETE FROM recordatorios_enviados WHERE regla_id IN (SELECT id FROM reglas_recordatorio WHERE user_id = $1)",
        "DELETE FROM reglas_recordatorio WHERE user_id = $1",
        "DELETE FROM campanas WHERE user_id = $1",
        "DELETE FROM plantillas_whatsapp WHERE user_id = $1",
        "DELETE FROM clientes_etiquetas WHERE cliente_id IN (SELECT id FROM clientes WHERE user_id = $1)",
        "DELETE FROM reservas_etiquetas WHERE reserva_id IN (SELECT id FROM reservas WHERE user_id = $1)",
        "DELETE FROM reservas WHERE user_id = $1",
        "DELETE FROM clientes WHERE user_id = $1",
        "DELETE FROM canales_reserva WHERE user_id = $1",
        "DELETE FROM ventas WHERE user_id = $1",
        "DELETE FROM gastos WHERE user_id = $1",
    ];
    for sql in &sentencias {
        sqlx::query(sql)
            .bind(user_id)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("Error limpiando datos: {e}"));
    }
    println!("Datos anteriores limpiados.");
}

/* 6 canales de reserva tipicos de restaurante */
async fn seed_canales(pool: &PgPool, user_id: Uuid) -> Vec<Uuid> {
    let nombres = [
        "Teléfono", "WhatsApp", "Web", "Walk-in", "Google Maps", "Instagram",
    ];
    let mut ids = Vec::with_capacity(nombres.len());
    for nombre in &nombres {
        let id: Uuid = sqlx::query_scalar!(
            "INSERT INTO canales_reserva (user_id, nombre) VALUES ($1, $2) RETURNING id",
            user_id,
            *nombre
        )
        .fetch_one(pool)
        .await
        .expect("Error al insertar canal");
        ids.push(id);
    }
    println!("  {} canales insertados.", ids.len());
    ids
}

/* 18 clientes demo con datos variados */
#[allow(clippy::type_complexity)]
async fn seed_clientes(pool: &PgPool, user_id: Uuid) -> Vec<Uuid> {
    /* (nombre, apellidos, telefono, email, empresa, alergias, pref_bebida, pref_ubicacion) */
    let datos: &[(&str, &str, &str, &str, &str, &str, &str, &str)] = &[
        ("María", "García López", "612345678", "maria.garcia@email.com", "", "Frutos secos", "Vino tinto", ""),
        ("Carlos", "Rodríguez Pérez", "623456789", "carlos.rod@email.com", "Deloitte", "", "Cerveza artesanal", ""),
        ("Ana", "Martínez Sánchez", "634567890", "", "", "Celiaca", "", ""),
        ("Pedro", "Sánchez Ruiz", "645678901", "pedro.s@email.com", "", "", "", "Ventana"),
        ("Laura", "Fernández Díaz", "656789012", "laura.f@email.com", "Accenture", "Lactosa", "", ""),
        ("Javier", "López Torres", "667890123", "", "", "", "", "Terraza"),
        ("Carmen", "Ruiz Navarro", "678901234", "carmen.ruiz@email.com", "", "Vegetariana", "Agua con gas", ""),
        ("Miguel", "Torres Romero", "689012345", "", "Eventos Sol S.L.", "", "", "Salón privado"),
        ("Lucía", "Moreno Gil", "690123456", "lucia.m@email.com", "", "", "", ""),
        ("David", "Jiménez Molina", "601234567", "david.j@email.com", "", "Marisco", "Cocktails", ""),
        ("Sofía", "Navarro Serrano", "612345098", "", "", "", "", "Interior"),
        ("Pablo", "Romero Blanco", "623450987", "pablo.r@email.com", "Telefónica", "", "", ""),
        ("Elena", "Díaz Vázquez", "634509876", "", "", "Vegana", "Zumos naturales", ""),
        ("Sergio", "Muñoz Ramos", "645098765", "sergio.m@email.com", "", "", "", ""),
        ("Teresa", "Alonso Ibáñez", "656098754", "teresa.a@email.com", "", "Frutos rojos", "", ""),
        ("Raúl", "Gutiérrez Cano", "667098543", "", "", "", "", "Mesa alta"),
        ("Isabel", "Herrera Prieto", "678098432", "isabel.h@email.com", "Banco Santander", "", "Champagne", ""),
        ("Alejandro", "Castro Méndez", "689098321", "", "", "Gluten", "", ""),
    ];
    let mut ids = Vec::with_capacity(datos.len());
    for &(nombre, apellidos, tel, email, empresa, alergias, pref_beb, pref_ubi) in datos {
        let id: Uuid = sqlx::query_scalar!(
            "INSERT INTO clientes (user_id, nombre, apellidos, telefono, email, empresa, \
             alergias, preferencias_bebida, preferencias_ubicacion) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
            user_id,
            nombre,
            apellidos,
            tel,
            email,
            empresa,
            alergias,
            pref_beb,
            pref_ubi
        )
        .fetch_one(pool)
        .await
        .expect("Error al insertar cliente");
        ids.push(id);
    }
    println!("  {} clientes insertados.", ids.len());
    ids
}

async fn seed_ventas(pool: &PgPool, user_id: Uuid, desde: NaiveDate, hasta: NaiveDate) {
    let turnos = ["manana", "mediodia", "noche"];
    let canales = ["comedor", "barra", "terraza", "delivery", "just_eat", "eventos"];
    let metodos = ["efectivo", "tarjeta", "transferencia", "otros"];
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

            sqlx::query!(
                "INSERT INTO ventas (user_id, fecha, comensales, descripcion, iva_porcentaje, turno, canal, metodo_pago, importe_base, importe_iva) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                user_id,
                fecha,
                i32::try_from(comensales).unwrap_or(2),
                descripciones[(idx + i) % descripciones.len()],
                Decimal::from(10),
                turnos[(idx + i) % turnos.len()],
                canales[(idx + i) % canales.len()],
                metodos[(idx + i) % metodos.len()],
                base,
                iva
            )
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
    let categorias: Vec<Uuid> = sqlx::query_scalar!("SELECT id FROM categorias_gasto ORDER BY nombre")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let tipos = ["factura", "albaran", "ticket"];
    let metodos = ["efectivo", "tarjeta", "transferencia", "otros"];
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

            sqlx::query!(
                "INSERT INTO gastos (user_id, fecha, proveedor, categoria_id, tipo_documento, metodo_pago, numero_documento, recurrente, importe_base, importe_iva) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                user_id,
                fecha,
                proveedores[(idx + i) % proveedores.len()],
                cat_id,
                tipos[(idx + i) % tipos.len()],
                metodos[(idx + i) % metodos.len()],
                format!("DOC-{:04}", count + 1),
                (idx + i).is_multiple_of(5),
                base,
                iva
            )
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

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap, clippy::too_many_lines)]
async fn seed_reservas(
    pool: &PgPool,
    user_id: Uuid,
    hoy: NaiveDate,
    canal_ids: &[Uuid],
    cliente_ids: &[Uuid],
) {
    let nombres = [
        "María", "Carlos", "Ana", "Pedro", "Laura", "Javier", "Carmen",
        "Miguel", "Lucía", "David", "Sofía", "Pablo", "Elena", "Sergio",
        "Teresa", "Raúl",
    ];
    let apellidos = [
        "García", "Rodríguez", "Martínez", "Sánchez", "Fernández", "López",
        "Ruiz", "Torres", "Moreno", "Jiménez", "Navarro", "Romero",
        "Díaz", "Muñoz", "Alonso", "Gutiérrez",
    ];
    let horas = [
        NaiveTime::from_hms_opt(13, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(13, 30, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(14, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(14, 30, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(20, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(20, 30, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(21, 0, 0).unwrap_or_default(),
        NaiveTime::from_hms_opt(21, 30, 0).unwrap_or_default(),
    ];

    let mut count: u32 = 0;

    /* Reservas pasadas: ultimos 30 dias (para estadisticas de no-shows) */
    for dia in (1_u32..=30).rev() {
        let fecha = hoy - Duration::days(i64::from(dia));
        let num_reservas = 3 + ((30 - dia) as usize % 6);
        let dia_u = dia as usize;

        for i in 0..num_reservas {
            let idx = (dia_u * 7 + i) % nombres.len();
            let personas = 2 + i % 6;

            /* Distribucion: ~70% completada, ~15% no_show, ~15% cancelada */
            let (estado, no_show) = match (dia_u + i) % 20 {
                0 | 5 | 10 => ("no_show", true),
                3 | 8 | 13 => ("cancelada", false),
                _ => ("completada", false),
            };

            /* ~70% de reservas tienen canal asignado */
            let canal_id = if (dia_u + i) % 10 < 7 {
                Some(canal_ids[(dia_u + i) % canal_ids.len()])
            } else {
                None
            };

            /* ~40% vinculadas a un cliente del CRM */
            let (cliente_id, nombre_cli, apellido_cli) = if (dia_u + i) % 5 < 2 {
                let ci = (dia_u + i) % cliente_ids.len();
                (Some(cliente_ids[ci]), nombres[ci % nombres.len()], apellidos[ci % apellidos.len()])
            } else {
                (None, nombres[idx], apellidos[idx])
            };

            let num_mesa: Option<i32> = if (dia_u + i).is_multiple_of(3) {
                Some(1 + ((dia_u + i) % 15) as i32)
            } else {
                None
            };

            let telefono = format!("6{:08}", 10_000_000 + count * 1234 + dia * 100);

            sqlx::query!(
                "INSERT INTO reservas (user_id, fecha, hora, nombre_cliente, apellidos_cliente, \
                 num_personas, estado, notas, telefono, canal_id, cliente_id, no_show, num_mesa) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
                user_id,
                fecha,
                horas[i % horas.len()],
                nombre_cli,
                apellido_cli,
                i32::try_from(personas).unwrap_or(2),
                estado,
                if i % 4 == 0 { "Mesa junto a ventana" } else { "" },
                &telefono,
                canal_id,
                cliente_id,
                no_show,
                num_mesa
            )
            .execute(pool)
            .await
            .expect("Error al insertar reserva pasada");

            count += 1;
        }
    }

    /* Reservas futuras: hoy + proximos 14 dias */
    for dia in 0_u32..15 {
        let fecha = hoy + Duration::days(i64::from(dia));
        let num_reservas = 3 + (dia as usize % 5);
        let dia_u = dia as usize;

        for i in 0..num_reservas {
            let idx = (dia_u * 7 + i) % nombres.len();
            let personas = 2 + i % 6;

            /* Futuras: ~60% confirmada, ~30% pendiente, ~10% lista_espera */
            let estado = match (dia_u + i) % 10 {
                0 => "lista_espera",
                1 | 4 | 7 => "pendiente",
                _ => "confirmada",
            };

            let canal_id = if (dia_u + i) % 10 < 7 {
                Some(canal_ids[(dia_u + i) % canal_ids.len()])
            } else {
                None
            };

            let (cliente_id, nombre_cli, apellido_cli) = if (dia_u + i) % 5 < 2 {
                let ci = (dia_u + i) % cliente_ids.len();
                (Some(cliente_ids[ci]), nombres[ci % nombres.len()], apellidos[ci % apellidos.len()])
            } else {
                (None, nombres[idx], apellidos[idx])
            };

            let num_mesa: Option<i32> = if (dia_u + i).is_multiple_of(3) {
                Some(1 + ((dia_u + i) % 15) as i32)
            } else {
                None
            };

            let telefono = format!("6{:08}", 10_000_000 + count * 1234 + dia * 100);

            sqlx::query!(
                "INSERT INTO reservas (user_id, fecha, hora, nombre_cliente, apellidos_cliente, \
                 num_personas, estado, notas, telefono, canal_id, cliente_id, no_show, num_mesa) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
                user_id,
                fecha,
                horas[i % horas.len()],
                nombre_cli,
                apellido_cli,
                i32::try_from(personas).unwrap_or(2),
                estado,
                if i % 3 == 0 { "Mesa junto a ventana" } else { "" },
                &telefono,
                canal_id,
                cliente_id,
                false,
                num_mesa
            )
            .execute(pool)
            .await
            .expect("Error al insertar reserva futura");

            count += 1;
        }
    }
    println!("  {count} reservas insertadas (pasadas + futuras).");
}

/* Asigna etiquetas del sistema a algunos clientes para probar el CRM */
async fn seed_etiquetas_clientes(pool: &PgPool, user_id: Uuid, cliente_ids: &[Uuid]) {
    /* Obtener etiquetas del sistema (es_sistema = TRUE, aplica_a = 'cliente') */
    let etiquetas = sqlx::query!(
        "SELECT e.id, e.nombre FROM etiquetas e \
         JOIN categorias_etiqueta ce ON ce.id = e.categoria_id \
         WHERE e.es_sistema = TRUE AND ce.aplica_a = 'cliente' \
         ORDER BY e.nombre"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    if etiquetas.is_empty() {
        println!("  Sin etiquetas del sistema — omitiendo asignaciones.");
        return;
    }

    let mut count: u32 = 0;

    /* Asignar 1-3 etiquetas a cada cliente segun patron determinista */
    for (ci, cliente_id) in cliente_ids.iter().enumerate() {
        let num_tags = 1 + ci % 3;
        for t in 0..num_tags {
            let etiqueta = &etiquetas[(ci + t) % etiquetas.len()];

            /* ON CONFLICT para evitar duplicados si se re-ejecuta */
            let resultado = sqlx::query!(
                "INSERT INTO clientes_etiquetas (cliente_id, etiqueta_id) \
                 VALUES ($1, $2) ON CONFLICT DO NOTHING",
                cliente_id,
                etiqueta.id
            )
            .execute(pool)
            .await;

            if resultado.is_ok() {
                count += 1;
            }
        }
    }
    println!("  {count} asignaciones cliente-etiqueta insertadas.");

    /* Tambien verificar que las etiquetas del usuario existan.
       Si no hay etiquetas propias, crear algunas del usuario demo
       para que el frontend las muestre como disponibles. */
    let user_tags: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM etiquetas WHERE user_id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .unwrap_or(None)
    .unwrap_or(0);

    if user_tags == 0 {
        /* Obtener categorias de sistema para usarlas como padre */
        let cat_fidelizacion: Option<Uuid> = sqlx::query_scalar!(
            "SELECT id FROM categorias_etiqueta \
             WHERE nombre = 'Fidelización' AND aplica_a = 'cliente' AND es_sistema = TRUE"
        )
        .fetch_optional(pool)
        .await
        .unwrap_or(None);

        if let Some(cat_id) = cat_fidelizacion {
            /* Crear etiquetas personalizadas del usuario sobre la categoria de sistema */
            let custom = [("Habitual fin de semana", "#2196F3"), ("Amigo del chef", "#9C27B0")];
            for (nombre, color) in &custom {
                let _ = sqlx::query!(
                    "INSERT INTO etiquetas (user_id, categoria_id, nombre, color) \
                     VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                    user_id,
                    cat_id,
                    *nombre,
                    *color
                )
                .execute(pool)
                .await;
            }
            println!("  Etiquetas personalizadas del usuario creadas.");
        }
    }
}

/* [263A-23] Campañas de marketing de prueba — 3 en distintos estados */
#[allow(clippy::type_complexity)]
async fn seed_campanas(pool: &PgPool, user_id: Uuid) {
    let campanas: Vec<(&str, &str, &[&str], &str, &str, &str)> = vec![
        (
            "Promoción verano 2026",
            "Campaña para recuperar clientes inactivos",
            &["sms", "email"],
            "sin_3m",
            "¡Te echamos de menos! Vuelve este verano y disfruta de un 15% de descuento en tu próxima reserva. Reserva ya.",
            "borrador",
        ),
        (
            "Menú degustación mayo",
            "Promoción nuevo menú degustación primavera",
            &["email"],
            "habitual",
            "Hola, hemos preparado un nuevo menú degustación de temporada. Como cliente habitual, tienes acceso prioritario. Reserva tu mesa.",
            "enviada",
        ),
        (
            "Recordatorio San Valentín",
            "Campaña cancelada — se planificó tarde",
            &["sms", "whatsapp"],
            "todos",
            "Celebra San Valentín con nosotros. Menú especial para 2 por 65€. Reservas: 912345678",
            "cancelada",
        ),
    ];

    for (nombre, desc, canales, segmento, mensaje, estado) in &campanas {
        let canales_arr: Vec<String> = canales.iter().map(|c| (*c).to_string()).collect();
        let total_dest: i32 = if *estado == "enviada" { 42 } else { 0 };
        let total_env: i32 = if *estado == "enviada" { 38 } else { 0 };
        let total_fall: i32 = if *estado == "enviada" { 4 } else { 0 };

        sqlx::query!(
            "INSERT INTO campanas (user_id, nombre, descripcion_interna, cuerpo_mensaje, \
             canales, segmento, estado, total_destinatarios, total_enviados, total_fallidos) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            user_id,
            *nombre,
            *desc,
            *mensaje,
            &canales_arr,
            *segmento,
            *estado,
            total_dest,
            total_env,
            total_fall
        )
        .execute(pool)
        .await
        .expect("Error al insertar campaña de prueba");
    }
    println!("  {} campañas de marketing insertadas.", campanas.len());
}

/* [263A-24] Plantillas WhatsApp de prueba — 3 en distintos estados */
#[allow(clippy::type_complexity)]
async fn seed_plantillas_whatsapp(pool: &PgPool, user_id: Uuid) {
    /* (nombre, categoria, idioma, cuerpo, estado, meta_template_id, meta_razon_rechazo) */
    let plantillas: Vec<(&str, &str, &str, &str, &str, Option<&str>, Option<&str>)> = vec![
        (
            "promocion_verano",
            "MARKETING",
            "es",
            "¡Hola {{1}}! Este verano disfruta de un 15% de descuento en tu próxima reserva. Te esperamos. Reserva ya en {{2}}.",
            "aprobada",
            Some("tpl_abc123"),
            None,
        ),
        (
            "confirmacion_reserva",
            "UTILITY",
            "es",
            "Hola {{1}}, tu reserva para {{2}} personas el {{3}} a las {{4}} está confirmada. ¡Te esperamos!",
            "borrador",
            None,
            None,
        ),
        (
            "oferta_navidad",
            "MARKETING",
            "es",
            "¡Felices fiestas {{1}}! Celebra la Navidad con nuestro menú especial por 45€/persona. Reserva: {{2}}",
            "rechazada",
            Some("tpl_xyz789"),
            Some("El contenido promocional no cumple la política de precios de Meta."),
        ),
    ];

    for (nombre, categoria, idioma, cuerpo, estado, meta_id, razon) in &plantillas {
        sqlx::query!(
            "INSERT INTO plantillas_whatsapp (user_id, nombre, categoria, idioma, cuerpo_mensaje, \
             estado, meta_template_id, meta_razon_rechazo) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            user_id,
            *nombre,
            *categoria,
            *idioma,
            *cuerpo,
            *estado,
            *meta_id,
            *razon
        )
        .execute(pool)
        .await
        .expect("Error al insertar plantilla WhatsApp de prueba");
    }
    println!("  {} plantillas WhatsApp insertadas.", plantillas.len());
}

/* [263A-25] Reglas de recordatorio de reservas para demo.
 * 3 reglas: SMS 24h antes, WhatsApp 1h antes, Email 24h (inactiva). */
async fn seed_reglas_recordatorio(pool: &PgPool, user_id: Uuid) {
    /* (nombre, horas_antes, canal, mensaje_plantilla, activa) */
    let reglas: Vec<(&str, i32, &str, &str, bool)> = vec![
        (
            "Recordatorio 24h antes por SMS",
            24,
            "sms",
            "Hola {nombre}, te recordamos tu reserva mañana a las {hora}. ¡Te esperamos!",
            true,
        ),
        (
            "Recordatorio 1h antes por WhatsApp",
            1,
            "whatsapp",
            "Hola {nombre}, tu reserva es en 1 hora ({hora}). ¿Sigues confirmado/a?",
            true,
        ),
        (
            "Email día anterior",
            24,
            "email",
            "Estimado/a {nombre}, le recordamos que tiene una reserva programada para mañana a las {hora}. Si necesita cancelar o modificar, responda a este email.",
            false,
        ),
    ];

    for (nombre, horas_antes, canal, mensaje, activa) in &reglas {
        sqlx::query!(
            "INSERT INTO reglas_recordatorio (user_id, nombre, horas_antes, canal, mensaje_plantilla, activa) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            user_id,
            *nombre,
            *horas_antes,
            *canal,
            *mensaje,
            *activa
        )
        .execute(pool)
        .await
        .expect("Error al insertar regla de recordatorio de prueba");
    }
    println!("  {} reglas de recordatorio insertadas.", reglas.len());
}
