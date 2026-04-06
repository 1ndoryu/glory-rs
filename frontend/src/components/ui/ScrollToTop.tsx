/* [064A-6] Scroll al tope en cada cambio de ruta.
 * Se monta dentro de BrowserRouter para que useLocation funcione. */

import {useEffect} from 'react';
import {useLocation} from 'react-router-dom';

export function ScrollToTop() {
    const {pathname} = useLocation();

    useEffect(() => {
        window.scrollTo(0, 0);
    }, [pathname]);

    return null;
}
