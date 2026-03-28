# 283A-34 — Filtros en gastos + Gráficos fix

## Tarea 25: Filtros en ListaGastos
- Creado `useListaGastos.ts` hook con filtros desde/hasta + paginación + modales
- ListaGastos ahora muestra date pickers para filtrar por rango de fechas (el backend ya soportaba `desde`, `hasta` y `categoria_id`)
- 0 useState en el componente (todo en hook)

## Tarea 28: Gráficos
- Agregado `width={30}` a todos los `<YAxis>` (5 instancias en DashboardReservas) para reducir espacio izquierdo
- Agregado `<Legend />` al PieChart de "Distribución por canal" con import de recharts
