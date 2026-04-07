/* [064A-32] Página de Hosting Administrado.
 * Reemplaza el placeholder de /soluciones/hosting con página funcional.
 * Muestra planes, features y CTA (abre chat — flujo Stripe subscription pendiente). */

import {useTranslation} from 'react-i18next';
import {Server, Shield, Zap, Clock, Globe, Headphones} from 'lucide-react';
import {LayoutPagina} from '../components/layout/LayoutPagina';
import {SEOHead} from '../components/seo/SEOHead';
import {SeccionContacto} from '../components/home/SeccionContacto';
import {useChatStore} from '../stores/chatStore';
import {PLANES_HOSTING} from '../data/planes/planes-hosting';
import {Button} from '../components/ui/Button';
import '../components/servicios/SeccionPlanesServicio.css';
import './SolucionHostingIsland.css';

const FEATURES = [
    {icono: Zap, titulo: 'Alto Rendimiento', desc: 'Servidores SSD con última generación de hardware para tiempos de carga mínimos.'},
    {icono: Shield, titulo: 'Seguridad Incluida', desc: 'SSL gratuito, firewall, backups automáticos y monitoreo 24/7.'},
    {icono: Clock, titulo: '99.9% Uptime', desc: 'Infraestructura redundante con failover automático para máxima disponibilidad.'},
    {icono: Globe, titulo: 'CDN Global', desc: 'Red de distribución de contenido para velocidad óptima desde cualquier ubicación.'},
    {icono: Server, titulo: 'Administrado', desc: 'Actualizaciones, parches y optimizaciones sin que toques un servidor.'},
    {icono: Headphones, titulo: 'Soporte Experto', desc: 'Equipo técnico disponible para resolver cualquier incidencia rápidamente.'},
];

export const SolucionHostingIsland = (): JSX.Element => {
    const {t} = useTranslation();
    const abrirChat = useChatStore(s => s.abrir);

    return (
        <LayoutPagina className="hostingPaginaMain">
            <SEOHead
                title="Hosting Administrado"
                description="Hosting de alto rendimiento con soporte experto. Planes desde $15/mes con SSL, backups y CDN incluidos."
                path="/soluciones/hosting"
            />

            {/* Hero */}
            <section className="hostingHero">
                <div className="hostingHeroContenido">
                    <span className="hostingHeroEtiqueta">Hosting Administrado</span>
                    <h1 className="hostingHeroTitulo">
                        Infraestructura que escala contigo
                    </h1>
                    <p className="hostingHeroDesc">
                        Olvídate de la administración de servidores. Nos encargamos de todo:
                        rendimiento, seguridad, backups y actualizaciones.
                    </p>
                    <div className="hostingHeroBotones">
                        <Button variante="primario" onClick={() => {
                            document.getElementById('planesHosting')?.scrollIntoView({behavior: 'smooth'});
                        }}>
                            Ver Planes
                        </Button>
                        <Button variante="outline" onClick={abrirChat}>
                            {t('plans.chat_with_us', 'Conversar')}
                        </Button>
                    </div>
                </div>
            </section>

            {/* Features */}
            <section className="hostingFeatures">
                <h2 className="hostingFeaturesTitle">Todo Incluido</h2>
                <p className="hostingFeaturesSubtitle">
                    Cada plan incluye las herramientas esenciales para mantener tu sitio rápido y seguro.
                </p>
                <div className="hostingFeaturesGrid">
                    {FEATURES.map((f) => (
                        <div key={f.titulo} className="hostingFeatureCard">
                            <f.icono size={24} strokeWidth={1.5} />
                            <h3>{f.titulo}</h3>
                            <p>{f.desc}</p>
                        </div>
                    ))}
                </div>
            </section>

            {/* Planes */}
            <section id="planesHosting" className="planesSeccion">
                <div className="planesContenedor">
                    <div className="planesCabecera">
                        <h2 className="planesTitulo">Planes de Hosting</h2>
                        <p className="planesSubtitulo">Elige el plan que mejor se adapte a tu proyecto</p>
                    </div>
                    <div className="planesGrid">
                        {PLANES_HOSTING.map(plan => (
                            <div key={plan.id} className={`tarjetaPlan ${plan.destacado ? 'tarjetaPlanDestacado' : ''}`}>
                                {plan.destacado && <div className="tarjetaPlanBadge">{t('plans.recommended')}</div>}
                                <div className="tarjetaPlanCabecera">
                                    <h3 className="tarjetaPlanNombre">{plan.nombre}</h3>
                                    <div className="tarjetaPlanPrecio">
                                        <span className="tarjetaPlanPrecioCifra">{plan.precio}</span>
                                        {plan.periodo && <span className="tarjetaPlanPrecioPeriodo">{plan.periodo}</span>}
                                    </div>
                                    <p className="tarjetaPlanDescripcion">{plan.descripcion}</p>
                                </div>
                                <ul className="tarjetaPlanCaracteristicas">
                                    {plan.caracteristicas.map((car, idx) => (
                                        <li key={idx} className={`tarjetaPlanItem ${car.incluido ? 'tarjetaPlanItemIncluido' : 'tarjetaPlanItemNoIncluido'}`}>
                                            <span className="tarjetaPlanItemIcono">
                                                {car.incluido ? (
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                                                        <polyline points="20 6 9 17 4 12" />
                                                    </svg>
                                                ) : (
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                                        <line x1="18" y1="6" x2="6" y2="18" />
                                                        <line x1="6" y1="6" x2="18" y2="18" />
                                                    </svg>
                                                )}
                                            </span>
                                            <span className="tarjetaPlanItemTexto">{car.texto}</span>
                                        </li>
                                    ))}
                                </ul>
                                <div className="tarjetaPlanAccion">
                                    <Button
                                        variante={plan.destacado ? 'primario' : 'outline'}
                                        tamano="mediano"
                                        onClick={abrirChat}
                                    >
                                        {plan.ctaTexto}
                                    </Button>
                                    <Button variante="texto" className="tarjetaPlanConversar" onClick={abrirChat}>
                                        {t('plans.chat_with_us', 'Conversar')}
                                    </Button>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </section>

            <SeccionContacto />
        </LayoutPagina>
    );
};
