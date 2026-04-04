/* [044A-28] Handlers SEO: robots.txt y sitemap.xml.
 * Endpoints estáticos que los crawlers necesitan para indexar correctamente. */
use axum::{response::IntoResponse, routing::get, Router};
use axum::http::header;

use crate::AppState;

const SITE_URL: &str = "https://nakomi.studio";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/robots.txt", get(robots_txt))
        .route("/sitemap.xml", get(sitemap_xml))
}

async fn robots_txt() -> impl IntoResponse {
    let body = format!(
        "User-agent: *\n\
         Allow: /\n\
         Disallow: /panel\n\
         Disallow: /swagger-ui\n\
         Disallow: /api-docs\n\n\
         Sitemap: {SITE_URL}/sitemap.xml"
    );
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}

async fn sitemap_xml() -> impl IntoResponse {
    /* Rutas estáticas del SPA — TO-DO: agregar dinámicas desde BD cuando haya contenido */
    let rutas = [
        ("/", "1.0", "weekly"),
        ("/servicios", "0.9", "weekly"),
        ("/proyectos", "0.9", "weekly"),
        ("/nosotros", "0.7", "monthly"),
        ("/blog", "0.8", "daily"),
        ("/soluciones", "0.8", "monthly"),
        ("/contacto", "0.6", "monthly"),
    ];

    let urls: String = rutas
        .iter()
        .map(|(path, priority, freq)| {
            format!(
                "  <url>\n\
                 \x20   <loc>{SITE_URL}{path}</loc>\n\
                 \x20   <changefreq>{freq}</changefreq>\n\
                 \x20   <priority>{priority}</priority>\n\
                 \x20 </url>"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
         {urls}\n\
         </urlset>"
    );

    ([(header::CONTENT_TYPE, "application/xml; charset=utf-8")], xml)
}
