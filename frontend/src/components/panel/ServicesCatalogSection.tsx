import {GloryLink} from '../../core/router';
import {obtenerPlanesServicio} from '../../data/planes';
import {SERVICIOS_DATA} from '../../data/servicios';
import './ServicesCatalogSection.css';

interface ServicesCatalogSectionProps {
    mode: 'client' | 'admin';
}

export function ServicesCatalogSection({mode}: ServicesCatalogSectionProps) {
    const services = SERVICIOS_DATA.map(service => {
        const slug = service.link.split('/').filter(Boolean).pop() ?? '';
        const planes = obtenerPlanesServicio(slug)?.planes ?? [];

        return {
            ...service,
            slug,
            planes,
        };
    });

    const totalPlans = services.reduce((accumulator, service) => accumulator + service.planes.length, 0);
    const featuredPlans = services.reduce(
        (accumulator, service) => accumulator + service.planes.filter(plan => plan.destacado).length,
        0,
    );

    return (
        <section className="servicesCatalogSection">
            <div className="servicesCatalogSummary">
                <MetricCard label="Servicios activos" value={String(services.length)} />
                <MetricCard label="Planes visibles" value={String(totalPlans)} />
                <MetricCard label="Planes destacados" value={String(featuredPlans)} />
                <MetricCard
                    label="Modo"
                    value={mode === 'admin' ? 'Catalogo admin' : 'Catalogo cliente'}
                />
            </div>

            <div className="servicesCatalogIntro">
                <h2 className="servicesCatalogTitle">
                    {mode === 'admin' ? 'Catalogo actual de servicios' : 'Servicios y planes disponibles'}
                </h2>
                <p className="servicesCatalogText">
                    {mode === 'admin'
                        ? 'Esta vista resume el catalogo actual publicado con sus planes y slugs activos. El CRUD administrativo aun no existe en backend, pero la seccion ya no queda vacia.'
                        : 'Aqui puedes revisar las lineas de servicio activas, sus categorias y los planes asociados para ampliar o contratar nuevos trabajos.'}
                </p>
            </div>

            <div className="servicesCatalogGrid">
                {services.map(service => (
                    <article key={service.id} className="servicesCatalogCard">
                        <div className="servicesCatalogCardHeader">
                            <div>
                                <h3 className="servicesCatalogCardTitle">{service.titulo}</h3>
                                <p className="servicesCatalogCardSlug">/{service.slug}</p>
                            </div>
                            <span className="servicesCatalogCardCount">
                                {service.planes.length} plan{service.planes.length === 1 ? '' : 'es'}
                            </span>
                        </div>

                        <p className="servicesCatalogCardDescription">{service.descripcion}</p>

                        <div className="servicesCatalogTags">
                            {service.categorias.map(category => (
                                <span key={`${service.id}-${category}`} className="servicesCatalogTag">
                                    {category}
                                </span>
                            ))}
                            {service.skills && service.skills.length > 0 && (
                                <span className="servicesCatalogTag servicesCatalogTag--muted">
                                    {service.skills.length} skills
                                </span>
                            )}
                        </div>

                        <div className="servicesCatalogPlans">
                            {service.planes.length > 0 ? (
                                service.planes.map(plan => (
                                    <div key={plan.id} className="servicesCatalogPlanRow">
                                        <div>
                                            <p className="servicesCatalogPlanName">{plan.nombre}</p>
                                            <p className="servicesCatalogPlanMeta">
                                                {plan.precio}
                                                {plan.periodo ? ` / ${plan.periodo}` : ''}
                                            </p>
                                        </div>
                                        {plan.destacado && (
                                            <span className="servicesCatalogPlanBadge">Destacado</span>
                                        )}
                                    </div>
                                ))
                            ) : (
                                <p className="servicesCatalogEmptyPlans">Sin planes publicados para este slug.</p>
                            )}
                        </div>

                        <GloryLink className="servicesCatalogLink" to={service.link}>
                            Abrir ficha publica
                        </GloryLink>
                    </article>
                ))}
            </div>
        </section>
    );
}

function MetricCard({label, value}: {label: string; value: string}) {
    return (
        <article className="servicesCatalogMetricCard">
            <span className="servicesCatalogMetricLabel">{label}</span>
            <strong className="servicesCatalogMetricValue">{value}</strong>
        </article>
    );
}