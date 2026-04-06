/* [064A-3] Cabecera de columna reutilizable con ordenamiento + filtro por columna.
 * Renderiza el título, flecha de sort y un botón de filtro (Popover con checkboxes).
 * El filtro se muestra solo cuando se pasan `options`. Soporta multi-selección.
 * Indicador visual (punto azul) cuando hay filtro activo. */

import { Filter } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Checkbox } from '@/components/ui/checkbox';

interface FilterOption {
  value: string;
  label: string;
}

interface ColumnFilterHeaderProps {
  title: string;
  sortKey?: string;
  currentSortBy?: string;
  currentSortOrder?: 'asc' | 'desc';
  onSort?: () => void;
  options?: FilterOption[];
  selectedValues?: string[];
  onFilterChange?: (values: string[]) => void;
  className?: string;
}

function ColumnFilterHeader({
  title,
  sortKey,
  currentSortBy,
  currentSortOrder,
  onSort,
  options,
  selectedValues = [],
  onFilterChange,
  className = '',
}: ColumnFilterHeaderProps) {
  const isSorted = sortKey && currentSortBy === sortKey;
  const hasActiveFilter = selectedValues.length > 0;

  const toggleValue = (value: string) => {
    if (!onFilterChange) return;
    const next = selectedValues.includes(value)
      ? selectedValues.filter((v) => v !== value)
      : [...selectedValues, value];
    onFilterChange(next);
  };

  const clearFilter = () => {
    onFilterChange?.([]);
  };

  return (
    <div className={`flex items-center gap-1 ${className}`}>
      {onSort ? (
        <Button
          variant="ghost"
          className="h-auto cursor-pointer select-none p-0 text-left font-medium hover:bg-transparent hover:underline"
          onClick={onSort}
        >
          {title} {isSorted && (currentSortOrder === 'asc' ? '↑' : '↓')}
        </Button>
      ) : (
        <span className="font-medium">{title}</span>
      )}

      {options && options.length > 0 && onFilterChange && (
        <Popover>
          <PopoverTrigger asChild>
            <Button variant="ghost" size="icon" className="relative size-6">
              <Filter className="size-3.5" />
              {hasActiveFilter && (
                <span className="absolute -right-0.5 -top-0.5 size-2 rounded-full bg-primary" />
              )}
            </Button>
          </PopoverTrigger>
          <PopoverContent align="start" className="w-48 p-2">
            <div className="flex flex-col gap-1">
              {options.map((opt) => (
                <label
                  key={opt.value}
                  className="flex cursor-pointer items-center gap-2 rounded px-2 py-1.5 text-sm hover:bg-muted"
                >
                  <Checkbox
                    checked={selectedValues.includes(opt.value)}
                    onCheckedChange={() => toggleValue(opt.value)}
                  />
                  {opt.label}
                </label>
              ))}
              {hasActiveFilter && (
                <Button
                  variant="ghost"
                  size="sm"
                  className="mt-1 h-7 text-xs"
                  onClick={clearFilter}
                >
                  Limpiar filtro
                </Button>
              )}
            </div>
          </PopoverContent>
        </Popover>
      )}
    </div>
  );
}

export default ColumnFilterHeader;
