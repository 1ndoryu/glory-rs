import { useEffect, useRef, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

const routesWithSearch = ['/colecciones', '/musica', '/descubrir'];

export function useNavPublico() {
  const location = useLocation();
  const navigate = useNavigate();
  const debounceRef = useRef<ReturnType<typeof setTimeout>>();
  const currentSearch = new URLSearchParams(location.search).get('buscar') ?? '';
  const [searchValue, setSearchValue] = useState(currentSearch);
  const showSearch = routesWithSearch.some((route) => location.pathname.startsWith(route));

  useEffect(() => {
    setSearchValue(currentSearch);
  }, [currentSearch]);

  useEffect(() => {
    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, []);

  const updateSearch = (value: string) => {
    setSearchValue(value);
    if (!showSearch) {
      return;
    }
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }
    debounceRef.current = setTimeout(() => {
      const params = new URLSearchParams(location.search);
      if (value.trim()) {
        params.set('buscar', value.trim());
      } else {
        params.delete('buscar');
      }
      const nextSearch = params.toString();
      navigate(`${location.pathname}${nextSearch ? `?${nextSearch}` : ''}`, { replace: true });
    }, 300);
  };

  const clearSearch = () => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }
    setSearchValue('');
    const params = new URLSearchParams(location.search);
    params.delete('buscar');
    const nextSearch = params.toString();
    navigate(`${location.pathname}${nextSearch ? `?${nextSearch}` : ''}`, { replace: true });
  };

  return {
    clearSearch,
    searchValue,
    showSearch,
    updateSearch,
  };
}
