/* [283A-26] ErrorBoundary global — captura errores de React no manejados.
 * Muestra un fallback con opción de recargar o reportar el error al backend.
 * El reporte se envía por email al admin vía POST /api/reportar-error. */

import { Component, type ErrorInfo, type ReactNode } from 'react';
import { toast } from 'sonner';

interface Props { children: ReactNode }
interface State { hasError: boolean; error: Error | null }

class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary:', error, errorInfo);
  }

  handleReport = async () => {
    const { error } = this.state;
    if (!error) return;
    try {
      const token = localStorage.getItem('token');
      const res = await fetch('/api/reportar-error', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({
          mensaje: error.message,
          stack: error.stack ?? null,
          url: window.location.href,
          navegador: navigator.userAgent,
        }),
      });
      if (res.ok) {
        toast.success('Error reportado correctamente');
      } else {
        toast.error('No se pudo enviar el reporte');
      }
    } catch {
      toast.error('No se pudo enviar el reporte');
    }
  };

  handleReload = () => window.location.reload();

  render() {
    if (this.state.hasError) {
      return (
        <div className="flex flex-col items-center justify-center min-h-screen gap-4 p-8 text-center">
          <h1 className="text-xl font-bold">Algo salió mal</h1>
          <p className="text-muted-foreground max-w-lg">{this.state.error?.message}</p>
          <div className="flex gap-2">
            <button
              type="button"
              className="px-4 py-2 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90"
              onClick={this.handleReload}
            >Recargar página</button>
            <button
              type="button"
              className="px-4 py-2 rounded-md border border-input bg-background text-sm font-medium hover:bg-accent"
              onClick={this.handleReport}
            >Reportar error</button>
          </div>
        </div>
      );
    }
    return this.props.children;
  }
}

export default ErrorBoundary;
