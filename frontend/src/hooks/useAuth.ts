/* [174A-106] Hook useAuth contra el backend Axum.
 *
 * Reemplaza al `useAuth` del legado (que mezclaba cookies WordPress
 * con localStorage). Ahora todo va contra `/api/auth/{login,logout,refresh,register}`
 * y `/api/users/me` usando los hooks generados por Orval.
 *
 * Convención de tokens:
 *   - access token  → localStorage.token   (lo lee el axios-instance)
 *   - refresh token → localStorage.refresh
 *
 * Si en el futuro migramos a cookies HttpOnly, este hook es el único
 * lugar que cambia (axios-instance ya no leería de localStorage).
 */

import { useCallback } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import {
  useLogin,
  useLogout,
  useRegister,
} from '../api/generated/auth/auth';
import { getMeQueryKey, useMe } from '../api/generated/users/users';
import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
} from '../api/generated/model';

const TOKEN_KEY = 'token';
const REFRESH_KEY = 'refresh';

function persistTokens(payload: AuthResponse | undefined) {
  if (!payload) return;
  if (payload.token) localStorage.setItem(TOKEN_KEY, payload.token);
  if (payload.refresh_token) localStorage.setItem(REFRESH_KEY, payload.refresh_token);
}

function clearTokens() {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(REFRESH_KEY);
}

function unwrap<T>(response: unknown): T | undefined {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const payload: any = response;
  return (payload?.data ?? payload) as T | undefined;
}

export function useAuth() {
  const queryClient = useQueryClient();
  const hasToken = typeof window !== 'undefined' && !!localStorage.getItem(TOKEN_KEY);

  const meQuery = useMe({
    query: { enabled: hasToken, retry: false, refetchOnWindowFocus: false },
  });
  const loginMutation = useLogin();
  const logoutMutation = useLogout();
  const registerMutation = useRegister();

  const login = useCallback(
    async (body: LoginRequest) => {
      const res = await loginMutation.mutateAsync({ data: body });
      persistTokens(unwrap<AuthResponse>(res));
      await queryClient.invalidateQueries({ queryKey: getMeQueryKey() });
    },
    [loginMutation, queryClient],
  );

  const register = useCallback(
    async (body: RegisterRequest) => {
      const res = await registerMutation.mutateAsync({ data: body });
      persistTokens(unwrap<AuthResponse>(res));
      await queryClient.invalidateQueries({ queryKey: getMeQueryKey() });
    },
    [registerMutation, queryClient],
  );

  const logout = useCallback(async () => {
    const refreshToken = localStorage.getItem(REFRESH_KEY) ?? '';
    try {
      await logoutMutation.mutateAsync({ data: { refresh_token: refreshToken } });
    } catch {
      /* Ignoramos errores: el cleanup local debe ejecutarse igual. */
    }
    clearTokens();
    queryClient.removeQueries({ queryKey: getMeQueryKey() });
  }, [logoutMutation, queryClient]);

  return {
    user: unwrap<{ id: number; username: string; email?: string }>(meQuery.data),
    isAuthenticated: hasToken && meQuery.isSuccess,
    isLoading: meQuery.isLoading,
    login,
    register,
    logout,
    error:
      loginMutation.error ?? registerMutation.error ?? logoutMutation.error ?? meQuery.error,
  };
}
