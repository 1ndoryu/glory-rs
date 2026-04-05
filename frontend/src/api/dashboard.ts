/* [044A-38 Fase 10] API client del dashboard admin.
 * Endpoint único que retorna métricas consolidadas. */

import axiosInstance from './axios-instance';

/* ========== Types ========== */

export interface RevenueStats {
  total_revenue: number;
  monthly_revenue: number;
  held_amount: number;
  refunded_amount: number;
}

export interface OrderCounts {
  total: number;
  active: number;
  completed: number;
  cancelled: number;
  awaiting_assignment: number;
}

export interface EmployeePerformance {
  employee_id: string;
  email: string;
  active_orders: number;
  completed_orders: number;
  average_rating: number | null;
}

export interface DashboardAlerts {
  unassigned_orders: number;
  pending_refunds: number;
  overdue_orders: number;
  unread_admin_notifications: number;
}

export interface DashboardResponse {
  revenue: RevenueStats;
  orders: OrderCounts;
  employees: EmployeePerformance[];
  alerts: DashboardAlerts;
}

/* ========== REST Function ========== */

export async function apiGetDashboard(): Promise<DashboardResponse> {
  const { data } = await axiosInstance.get<DashboardResponse>('/admin/dashboard');
  return data;
}
