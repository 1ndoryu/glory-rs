/* Utilidad para generar el schema OpenAPI sin necesitar base de datos.
 * Uso: cargo run --bin dump_openapi > frontend/openapi-debug.json */

use glory_backend::handlers::ApiDoc;
use utoipa::OpenApi;

fn main() {
    print!(
        "{}",
        ApiDoc::openapi()
            .to_json()
            .expect("Error serializando OpenAPI")
    );
}
