/* [074A-6] API client para upload de imágenes CMS.
 * Envía multipart/form-data al endpoint admin-only. */
import instance from './axios-instance';

export interface UploadResponse {
    url: string;
    file_name: string;
}

export async function apiUploadImage(file: File): Promise<UploadResponse> {
    const formData = new FormData();
    formData.append('file', file);
    const { data } = await instance.post<UploadResponse>('/api/admin/uploads', formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
    });
    return data;
}
