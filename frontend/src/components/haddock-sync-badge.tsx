/* [064A-9] Badge de estado de sincronización Haddock.
 * Muestra estado visual (check/error/pendiente) con tooltip informativo.
 * Extraído de ListaVentas para mantener componente principal bajo 300 líneas. */

import { Check, X, Minus } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';

interface Props {
  synced: boolean;
  syncedAt?: string | null;
  syncError?: string | null;
}

function HaddockSyncBadge({ synced, syncedAt, syncError }: Props) {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          {synced ? (
            <Badge variant="outline" className="bg-green-50 text-green-700 border-green-200 text-xs">
              <Check className="size-3" />
            </Badge>
          ) : syncError ? (
            <Badge variant="outline" className="bg-red-50 text-red-700 border-red-200 text-xs">
              <X className="size-3" />
            </Badge>
          ) : (
            <Badge variant="outline" className="text-muted-foreground text-xs">
              <Minus className="size-3" />
            </Badge>
          )}
        </TooltipTrigger>
        <TooltipContent side="left" className="max-w-xs">
          {synced
            ? `Sincronizada${syncedAt ? ` el ${new Date(syncedAt).toLocaleString('es-ES')}` : ''}`
            : syncError
              ? `Error: ${syncError}`
              : 'No sincronizada'}
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}

export default HaddockSyncBadge;
