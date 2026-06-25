/* [256A-1c] Stub para @capacitor/filesystem — no disponible en desktop Tauri */
export const Directory = {
    Data: 'DATA',
    Documents: 'DOCUMENTS',
    Cache: 'CACHE',
};

export const Filesystem = {
    readFile: async (_options: { path: string; directory?: string }) => ({ data: '' }),
    writeFile: async (_options: { path: string; data: string; directory?: string; recursive?: boolean }) => ({ uri: '' }),
    deleteFile: async (_options: { path: string; directory?: string }) => {},
    mkdir: async (_options: { path: string; directory?: string; recursive?: boolean }) => {},
    readdir: async (_options: { path: string; directory?: string }) => ({ files: [] }),
    stat: async (_options: { path: string; directory?: string }) => ({ type: 'file', size: 0, mtime: 0 }),
};
