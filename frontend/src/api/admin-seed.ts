/* [064A-62] API client para gestión de datos de prueba (seed).
 * Solo admin. Permite recrear o borrar datos de test desde el panel. */

import axiosInstance from './axios-instance';

export interface SeedResponse {
  message: string;
}

/** Recrear datos de prueba (borra + crea nuevos) */
export async function apiRecreateSeed(): Promise<SeedResponse> {
  const { data } = await axiosInstance.post<SeedResponse>('/api/admin/seed');
  return data;
}

/** Borrar datos de prueba */
export async function apiDeleteSeed(): Promise<SeedResponse> {
  const { data } = await axiosInstance.delete<SeedResponse>('/api/admin/seed');
  return data;
}
