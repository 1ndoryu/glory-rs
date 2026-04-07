/* [074A-13] API client para CMS de equipo.
 * Mismo patrón que admin-projects.ts. */

import instance from './axios-instance';

export interface AdminTeamMember {
    id: string;
    name: string;
    slug: string;
    role: string;
    bio: string;
    avatar: string | null;
    linkedin: string | null;
    twitter: string | null;
    github: string | null;
    status: string;
    sort_order: number;
    created_at: string;
    updated_at: string;
}

export interface CreateTeamMemberBody {
    name: string;
    slug: string;
    role?: string;
    bio?: string;
    avatar?: string;
    linkedin?: string;
    twitter?: string;
    github?: string;
    status?: string;
    sort_order?: number;
}

export interface UpdateTeamMemberBody {
    name?: string;
    slug?: string;
    role?: string;
    bio?: string;
    avatar?: string;
    linkedin?: string;
    twitter?: string;
    github?: string;
    status?: string;
    sort_order?: number;
}

export async function apiListAdminTeamMembers(): Promise<AdminTeamMember[]> {
    const { data } = await instance.get<AdminTeamMember[]>('/admin/team');
    return data;
}

export async function apiCreateTeamMember(body: CreateTeamMemberBody): Promise<AdminTeamMember> {
    const { data } = await instance.post<AdminTeamMember>('/admin/team', body);
    return data;
}

export async function apiUpdateTeamMember(id: string, body: UpdateTeamMemberBody): Promise<AdminTeamMember> {
    const { data } = await instance.put<AdminTeamMember>(`/admin/team/${id}`, body);
    return data;
}

export async function apiArchiveTeamMember(id: string): Promise<void> {
    await instance.delete(`/admin/team/${id}`);
}

export async function apiListPublicTeamMembers(): Promise<AdminTeamMember[]> {
    const { data } = await instance.get<AdminTeamMember[]>('/team');
    return data;
}
