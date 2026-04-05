/* [044A-38 Fase 10] Hook del dashboard admin.
 * React Query con auto-refetch cada 30s para métricas en tiempo real. */

import { useQuery } from '@tanstack/react-query';
import { apiGetDashboard, type DashboardResponse } from '../api/dashboard';
import { useAuthStore } from '../stores/authStore';

const DASHBOARD_KEY = ['admin', 'dashboard'] as const;

export function useDashboard() {
  const token = useAuthStore((s) => s.token);

  const {
    data: dashboard,
    isLoading,
    error,
    refetch,
  } = useQuery<DashboardResponse>({
    queryKey: DASHBOARD_KEY,
    queryFn: apiGetDashboard,
    enabled: !!token,
    refetchInterval: 30_000,
    staleTime: 15_000,
  });

  return {
    dashboard,
    isLoading,
    error,
    refetch,
  };
}
