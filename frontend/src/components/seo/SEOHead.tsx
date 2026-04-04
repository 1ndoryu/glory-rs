/* [044A-28] Componente SEO reutilizable.
 * Gestiona title, meta description, Open Graph, Twitter Card, canonical
 * y structured data JSON-LD desde cada página/island. */
import {Helmet} from 'react-helmet-async';
import {useTranslation} from 'react-i18next';

const SITE_NAME = 'Nakomi Studio';
const SITE_URL = 'https://nakomi.studio';
const DEFAULT_IMAGE = `${SITE_URL}/og-image.jpg`;

interface SEOHeadProps {
    title?: string;
    description?: string;
    path?: string;
    image?: string;
    type?: 'website' | 'article';
    jsonLd?: Record<string, unknown>;
    noindex?: boolean;
}

export const SEOHead = ({
    title,
    description,
    path = '',
    image = DEFAULT_IMAGE,
    type = 'website',
    jsonLd,
    noindex = false,
}: SEOHeadProps) => {
    const {i18n} = useTranslation();
    const fullTitle = title ? `${title} | ${SITE_NAME}` : SITE_NAME;
    const canonicalUrl = `${SITE_URL}${path}`;

    return (
        <Helmet>
            <html lang={i18n.language} />
            <title>{fullTitle}</title>
            {description && <meta name="description" content={description} />}
            <link rel="canonical" href={canonicalUrl} />
            {noindex && <meta name="robots" content="noindex,nofollow" />}

            {/* Open Graph */}
            <meta property="og:title" content={fullTitle} />
            {description && <meta property="og:description" content={description} />}
            <meta property="og:url" content={canonicalUrl} />
            <meta property="og:type" content={type} />
            <meta property="og:image" content={image} />
            <meta property="og:site_name" content={SITE_NAME} />
            <meta property="og:locale" content={i18n.language} />

            {/* Twitter Card */}
            <meta name="twitter:card" content="summary_large_image" />
            <meta name="twitter:title" content={fullTitle} />
            {description && <meta name="twitter:description" content={description} />}
            <meta name="twitter:image" content={image} />

            {/* hreflang alternates */}
            <link rel="alternate" hrefLang="es" href={`${SITE_URL}${path}`} />
            <link rel="alternate" hrefLang="en" href={`${SITE_URL}${path}`} />
            <link rel="alternate" hrefLang="ja" href={`${SITE_URL}${path}`} />
            <link rel="alternate" hrefLang="x-default" href={`${SITE_URL}${path}`} />

            {/* Structured Data */}
            {jsonLd && (
                <script type="application/ld+json">
                    {JSON.stringify(jsonLd)}
                </script>
            )}
        </Helmet>
    );
};
