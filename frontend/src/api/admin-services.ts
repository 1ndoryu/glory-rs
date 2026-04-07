/* [074A-9] API client para CRUD de servicios desde panel admin.
 * Conecta con endpoints /api/admin/services del backend. */
import instance from './axios-instance';

export interface AdminServicePlan {
    id: string;
    slug: string;
    name: string;
    price_cents: number;
    description: string | null;
    features: unknown;
    is_highlighted: boolean;
    is_custom: boolean;
    phases: AdminPlanPhase[];
}

export interface AdminPlanPhase {
    phase_number: number;
    title: string;
    description: string | null;
    percentage_of_total: number;
    estimated_days: number;
    max_revisions: number;
}

export interface AdminService {
    id: string;
    slug: string;
    title: string;
    description: string | null;
    base_price_cents: number;
    currency: string;
    is_active: boolean;
    sort_order: number;
    image_url: string | null;
    gallery: string[];
    skills: string[];
    content: string | null;
    meta_title: string | null;
    meta_description: string | null;
    status: string;
    created_at: string;
    updated_at: string;
    plans: AdminServicePlan[];
}

export interface CreateServiceBody {
    title: string;
    slug: string;
    description?: string;
    base_price_cents?: number;
    currency?: string;
    image_url?: string;
    gallery?: string[];
    skills?: string[];
    content?: string;
    meta_title?: string;
    meta_description?: string;
    status?: string;
    sort_order?: number;
}

export interface UpdateServiceBody {
    title?: string;
    slug?: string;
    description?: string;
    base_price_cents?: number;
    currency?: string;
    is_active?: boolean;
    image_url?: string;
    gallery?: string[];
    skills?: string[];
    content?: string;
    meta_title?: string;
    meta_description?: string;
    status?: string;
    sort_order?: number;
}

export async function apiListAdminServices(): Promise<AdminService[]> {
    const {data} = await instance.get<AdminService[]>('/api/admin/services');
    return data;
}

export async function apiCreateService(body: CreateServiceBody): Promise<AdminService> {
    const {data} = await instance.post<AdminService>('/api/admin/services', body);
    return data;
}

export async function apiUpdateService(id: string, body: UpdateServiceBody): Promise<AdminService> {
    const {data} = await instance.put<AdminService>(`/api/admin/services/${id}`, body);
    return data;
}

export async function apiArchiveService(id: string): Promise<void> {
    await instance.delete(`/api/admin/services/${id}`);
}
