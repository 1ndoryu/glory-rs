import { useParams } from 'react-router-dom';

export default function SamplePage() {
  const { slug } = useParams<{ slug: string }>();
  return (
    <section className="pagina">
      <h1 className="titulo">Sample: {slug}</h1>
      <p className="descripcion">Pendiente migración (174A-104).</p>
    </section>
  );
}
