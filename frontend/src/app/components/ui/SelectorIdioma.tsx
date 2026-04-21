import { useIdiomaStore, type Idioma } from '../../utils/i18n';
import { useT } from '../../utils/i18n/useT';
import { SelectorBase } from './SelectorBase';
import '../../../styles/componentes/selectorIdioma.css';

interface SelectorIdiomaProps {
  variante?: 'compacto' | 'completo' | 'select';
  className?: string;
}

const IDIOMAS: Array<{
  id: Idioma;
  nombre: string;
}> = [
  { id: 'es', nombre: 'Español' },
  { id: 'en', nombre: 'English' },
  { id: 'ja', nombre: '日本語' },
];

export function SelectorIdioma({ variante = 'compacto', className = '' }: SelectorIdiomaProps): JSX.Element {
  const idioma = useIdiomaStore((state) => state.idioma);
  const setIdioma = useIdiomaStore((state) => state.setIdioma);
  const { t } = useT();

  return (
    <SelectorBase
      aria-label={t('idioma.seleccionar')}
      className={`selectorIdiomaMenu selectorIdiomaMenu--${variante}${className ? ` ${className}` : ''}`}
      onChange={(event) => setIdioma(event.target.value as Idioma)}
      value={idioma}
    >
      {IDIOMAS.map(({ id, nombre }) => (
        <option key={id} value={id}>{nombre}</option>
      ))}
    </SelectorBase>
  );
}

export default SelectorIdioma;