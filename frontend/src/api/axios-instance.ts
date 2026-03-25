import axios from 'axios';

const instance = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:3000',
});

/* Interceptor: agrega el token JWT a cada request si existe */
instance.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

/**
 * 253A-7: Mutator para Orval v8 — la firma cambió a (url, config) en v8.
 * Convierte RequestInit a AxiosRequestConfig y usa axios.
 */
export const customInstance = async <T>(
  url: string,
  config: RequestInit,
): Promise<T> => {
  const { body, headers, method, signal } = config;

  const response = await instance.request<T>({
    url,
    method: method as string,
    data: body,
    headers: headers as Record<string, string>,
    signal: signal ?? undefined,
  });

  return response.data;
};

export default instance;
