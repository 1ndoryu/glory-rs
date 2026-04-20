import { useParams } from 'react-router-dom';

export default function PerfilPage() {
  const { username } = useParams<{ username: string }>();
  return (
    <section className="pagina">
      <h1 className="titulo">Perfil: {username}</h1>
      <p className="descripcion">Pendiente migración (174A-104).</p>
    </section>
  );
}
