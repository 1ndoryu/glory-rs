/* [263A-16] DashboardReservas — reescrito con shadcn Card + Button + Tailwind.
 * 3 paneles con tabs: Resumen, Ocupación, Análisis.
 * [263A-25] Gráficos migrados a ChartContainer de shadcn — colores adaptativos dark/light. */

import { useState } from 'react';
import { useDashboardReservas, ResumenReservas, OcupacionReservas, AnalisisReservas, AgrupacionCanal } from '../api/generated';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { ChartContainer, ChartTooltip, ChartTooltipContent, type ChartConfig } from '@/components/ui/chart';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, PieChart, Pie, Cell } from 'recharts';

/* CSS vars del tema — se adaptan automáticamente a dark/light mode */
const PIE_COLORS = ['var(--chart-1)', 'var(--chart-2)', 'var(--chart-3)', 'var(--chart-4)', 'var(--chart-5)'];
const CONFIG_SIMPLE: ChartConfig = { total: { label: 'Reservas', color: 'var(--chart-1)' } };
const CONFIG_TURNO: ChartConfig = {
  total: { label: 'Reservas', color: 'var(--chart-1)' },
  personas: { label: 'Personas', color: 'var(--chart-2)' },
};

function DashboardReservas() {
  const ahora = new Date();
  const [anio, setAnio] = useState(ahora.getFullYear());
  const [mes, setMes] = useState(ahora.getMonth() + 1);
  const [pestana, setPestana] = useState<'resumen' | 'ocupacion' | 'analisis'>('resumen');

  const { data, isLoading } = useDashboardReservas({ year: anio, month: mes });
  const dashboard = data?.status === 200 ? data.data : null;

  const meses = [
    'Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio',
    'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre',
  ];

  return (
    <div className="flex flex-col gap-6">
      <div className="flex items-center gap-3">
        <Select value={String(mes)} onValueChange={v => setMes(Number(v))}>
          <SelectTrigger className="w-36"><SelectValue /></SelectTrigger>
          <SelectContent>
            {meses.map((nombre, i) => (
              <SelectItem key={i + 1} value={String(i + 1)}>{nombre}</SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Select value={String(anio)} onValueChange={v => setAnio(Number(v))}>
          <SelectTrigger className="w-24"><SelectValue /></SelectTrigger>
          <SelectContent>
            {[anio - 1, anio, anio + 1].map(a => (
              <SelectItem key={a} value={String(a)}>{a}</SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <div className="flex gap-1 border-b">
        {(['resumen', 'ocupacion', 'analisis'] as const).map(tab => (
          <Button
            key={tab}
            variant="ghost"
            size="sm"
            className={`rounded-b-none ${pestana === tab ? 'border-b-2 border-primary font-semibold' : ''}`}
            onClick={() => setPestana(tab)}
          >
            {tab === 'resumen' ? 'Resumen' : tab === 'ocupacion' ? 'Ocupación' : 'Análisis'}
          </Button>
        ))}
      </div>

      {isLoading && <p className="text-sm text-muted-foreground text-center py-8">Cargando dashboard...</p>}

      {dashboard && pestana === 'resumen' && <PanelResumen data={dashboard.resumen} />}
      {dashboard && pestana === 'ocupacion' && <PanelOcupacion data={dashboard.ocupacion} />}
      {dashboard && pestana === 'analisis' && <PanelAnalisis data={dashboard.analisis} />}
    </div>
  );
}

function PanelResumen({ data }: { data: ResumenReservas }) {
  const signoVariacion = data.variacion_porcentaje >= 0 ? '+' : '';

  return (
    <>
      <div className="grid gap-4 sm:grid-cols-2">
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Total reservas</CardTitle></CardHeader>
          <CardContent>
            <span className="text-2xl font-bold">{data.total_reservas}</span>
            <p className={`text-xs ${data.variacion_porcentaje >= 0 ? 'text-green-600' : 'text-destructive'}`}>
              {signoVariacion}{data.variacion_porcentaje}% vs mes anterior ({data.total_mes_anterior})
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Clientes nuevos</CardTitle></CardHeader>
          <CardContent><span className="text-2xl font-bold">{data.clientes_nuevos}</span></CardContent>
        </Card>
      </div>

      <div className="grid gap-4 lg:grid-cols-3">
        <Card>
          <CardHeader><CardTitle className="text-sm">Reservas por día</CardTitle></CardHeader>
          <CardContent>
            <ChartContainer config={CONFIG_SIMPLE} className="h-[220px]">
              <BarChart data={data.por_dia.map((d: { fecha: string; total: number }) => ({ ...d, fecha: d.fecha.slice(5) }))}>
                <CartesianGrid vertical={false} />
                <XAxis dataKey="fecha" tickLine={false} axisLine={false} fontSize={11} />
                <YAxis allowDecimals={false} tickLine={false} axisLine={false} />
                <ChartTooltip content={<ChartTooltipContent />} />
                <Bar dataKey="total" fill="var(--color-total)" radius={4} />
              </BarChart>
            </ChartContainer>
          </CardContent>
        </Card>

        <Card>
          <CardHeader><CardTitle className="text-sm">Por día de semana</CardTitle></CardHeader>
          <CardContent>
            <ChartContainer config={CONFIG_SIMPLE} className="h-[220px]">
              <BarChart data={data.por_dia_semana}>
                <CartesianGrid vertical={false} />
                <XAxis dataKey="dia" tickLine={false} axisLine={false} fontSize={11} />
                <YAxis allowDecimals={false} tickLine={false} axisLine={false} />
                <ChartTooltip content={<ChartTooltipContent />} />
                <Bar dataKey="total" fill="var(--color-total)" radius={4} />
              </BarChart>
            </ChartContainer>
          </CardContent>
        </Card>

        <Card>
          <CardHeader><CardTitle className="text-sm">Distribución por canal</CardTitle></CardHeader>
          <CardContent>
            {data.por_canal.length > 0 ? (
              <ChartContainer config={CONFIG_SIMPLE} className="h-[220px]">
                <PieChart>
                  <Pie data={data.por_canal} dataKey="total" nameKey="canal" cx="50%" cy="50%" outerRadius={80}>
                    {data.por_canal.map((_: AgrupacionCanal, i: number) => (
                      <Cell key={i} fill={PIE_COLORS[i % PIE_COLORS.length]} />
                    ))}
                  </Pie>
                  <ChartTooltip content={<ChartTooltipContent />} />
                </PieChart>
              </ChartContainer>
            ) : (
              <p className="text-sm text-muted-foreground text-center py-8">Sin datos de canales</p>
            )}
          </CardContent>
        </Card>
      </div>
    </>
  );
}

function PanelOcupacion({ data }: { data: OcupacionReservas }) {
  return (
    <>
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Media personas/reserva</CardTitle></CardHeader>
          <CardContent><span className="text-2xl font-bold">{data.media_personas}</span></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Media reservas/día</CardTitle></CardHeader>
          <CardContent><span className="text-2xl font-bold">{data.media_reservas_dia}</span></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Total reservas</CardTitle></CardHeader>
          <CardContent><span className="text-2xl font-bold">{data.total_reservas}</span></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Antelación media</CardTitle></CardHeader>
          <CardContent><span className="text-2xl font-bold">{data.antelacion_media_dias} días</span></CardContent>
        </Card>
      </div>

      <div className="grid gap-4 lg:grid-cols-3">
        <Card>
          <CardHeader><CardTitle className="text-sm">Distribución por hora</CardTitle></CardHeader>
          <CardContent>
            <ChartContainer config={CONFIG_SIMPLE} className="h-[220px]">
              <BarChart data={data.por_hora}>
                <CartesianGrid vertical={false} />
                <XAxis dataKey="hora" tickLine={false} axisLine={false} fontSize={11} />
                <YAxis allowDecimals={false} tickLine={false} axisLine={false} />
                <ChartTooltip content={<ChartTooltipContent />} />
                <Bar dataKey="total" fill="var(--color-total)" radius={4} />
              </BarChart>
            </ChartContainer>
          </CardContent>
        </Card>

        <Card>
          <CardHeader><CardTitle className="text-sm">Por turno</CardTitle></CardHeader>
          <CardContent>
            <ChartContainer config={CONFIG_TURNO} className="h-[220px]">
              <BarChart data={data.por_turno}>
                <CartesianGrid vertical={false} />
                <XAxis dataKey="turno" tickLine={false} axisLine={false} fontSize={11} />
                <YAxis allowDecimals={false} tickLine={false} axisLine={false} />
                <ChartTooltip content={<ChartTooltipContent />} />
                <Bar dataKey="total" fill="var(--color-total)" radius={4} />
                <Bar dataKey="personas" fill="var(--color-personas)" radius={4} />
              </BarChart>
            </ChartContainer>
          </CardContent>
        </Card>

        <Card>
          <CardHeader><CardTitle className="text-sm">Procedencia (canal)</CardTitle></CardHeader>
          <CardContent>
            <div className="rounded-md border">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Canal</TableHead>
                    <TableHead className="text-right">Reservas</TableHead>
                    <TableHead className="text-right">%</TableHead>
                    <TableHead></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {data.por_procedencia.map((c: AgrupacionCanal) => (
                    <TableRow key={c.canal}>
                      <TableCell>{c.canal}</TableCell>
                      <TableCell className="text-right">{c.total}</TableCell>
                      <TableCell className="text-right">{c.porcentaje}%</TableCell>
                      <TableCell>
                        <div className="h-2 w-full max-w-[100px] rounded-full bg-muted">
                          <div className="h-2 rounded-full bg-primary" style={{ width: `${Math.min(c.porcentaje, 100)}%` }} />
                        </div>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      </div>
    </>
  );
}

function PanelAnalisis({ data }: { data: AnalisisReservas }) {
  const formatMoneda = (v: string | null | undefined) => {
    if (!v) return '—';
    return new Intl.NumberFormat('es-ES', { style: 'currency', currency: 'EUR' }).format(parseFloat(v));
  };

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <Card>
        <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Reservas efectivas</CardTitle></CardHeader>
        <CardContent>
          <span className="text-2xl font-bold">{data.reservas_efectivas}</span>
          <p className="text-xs text-muted-foreground">Sin cancelaciones ni no-shows</p>
        </CardContent>
      </Card>
      <Card>
        <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Total comensales</CardTitle></CardHeader>
        <CardContent><span className="text-2xl font-bold">{data.total_comensales}</span></CardContent>
      </Card>
      <Card>
        <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Comensales/reserva</CardTitle></CardHeader>
        <CardContent><span className="text-2xl font-bold">{data.comensales_por_reserva}</span></CardContent>
      </Card>
      <Card>
        <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Ticket medio/reserva</CardTitle></CardHeader>
        <CardContent>
          <span className="text-2xl font-bold">{formatMoneda(data.ticket_medio_reserva)}</span>
          <p className="text-xs text-muted-foreground">Ventas ÷ reservas efectivas</p>
        </CardContent>
      </Card>
      <Card>
        <CardHeader className="pb-2"><CardTitle className="text-sm font-medium text-muted-foreground">Ticket medio/persona</CardTitle></CardHeader>
        <CardContent>
          <span className="text-2xl font-bold">{formatMoneda(data.ticket_medio_persona)}</span>
          <p className="text-xs text-muted-foreground">Ventas ÷ total comensales</p>
        </CardContent>
      </Card>
    </div>
  );
}

export default DashboardReservas;
