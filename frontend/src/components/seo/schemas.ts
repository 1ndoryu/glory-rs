/* [044A-28] Schemas JSON-LD para structured data.
 * Centraliza la generación de datos estructurados para Google rich snippets. */

const SITE_URL = 'https://nakomi.studio';

export const organizationSchema = {
    '@context': 'https://schema.org',
    '@type': 'ProfessionalService',
    name: 'Nakomi Studio',
    url: SITE_URL,
    logo: `${SITE_URL}/logo.png`,
    description: 'Agencia creativa especializada en desarrollo web, diseño UI/UX y soluciones digitales.',
    sameAs: [],
};

export const websiteSchema = {
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    name: 'Nakomi Studio',
    url: SITE_URL,
};

export const serviceSchema = (nombre: string, descripcion: string, slug: string) => ({
    '@context': 'https://schema.org',
    '@type': 'Service',
    name: nombre,
    description: descripcion,
    url: `${SITE_URL}/servicios/${slug}`,
    provider: {
        '@type': 'ProfessionalService',
        name: 'Nakomi Studio',
    },
});

export const blogPostSchema = (titulo: string, descripcion: string, slug: string, fecha?: string) => ({
    '@context': 'https://schema.org',
    '@type': 'BlogPosting',
    headline: titulo,
    description: descripcion,
    url: `${SITE_URL}/blog/${slug}`,
    ...(fecha && {datePublished: fecha}),
    author: {
        '@type': 'Organization',
        name: 'Nakomi Studio',
    },
    publisher: {
        '@type': 'Organization',
        name: 'Nakomi Studio',
    },
});

export const breadcrumbSchema = (items: {name: string; url: string}[]) => ({
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: items.map((item, i) => ({
        '@type': 'ListItem',
        position: i + 1,
        name: item.name,
        item: `${SITE_URL}${item.url}`,
    })),
});
