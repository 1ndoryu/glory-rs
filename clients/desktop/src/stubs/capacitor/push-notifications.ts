/* [256A-1c] Stub para @capacitor/push-notifications — no disponible en desktop Tauri */
export const PushNotifications = {
    register: async () => {},
    requestPermissions: async () => ({ granted: false, receive: 'denied' }),
    getDeliveredNotifications: async () => ({ notifications: [] }),
    addListener: async (_event: string, _callback: (...args: unknown[]) => void) => ({ remove: () => {} }),
    createChannel: async (_channel: Record<string, unknown>) => {},
};
