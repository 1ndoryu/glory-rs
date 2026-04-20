import { useParams } from 'react-router-dom';

export default function BlogPage() {
  const { slug } = useParams<{ slug?: string }>();
  return (
    <section className="pagina">
      <h1 className="titulo">{slug ? `Artículo: ${slug}` : 'Blog'}</h1>
      <p className="descripcion">Pendiente migración (174A-104).</p>
    </section>
  );
}
