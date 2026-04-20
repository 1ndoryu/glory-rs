import { Link } from 'react-router-dom';

export default function NotFoundPage() {
  return (
    <section className="pagina">
      <h1 className="titulo">404 — No encontrado</h1>
      <p className="descripcion">
        La ruta solicitada no existe. <Link to="/" className="enlace">Volver al inicio</Link>.
      </p>
    </section>
  );
}
