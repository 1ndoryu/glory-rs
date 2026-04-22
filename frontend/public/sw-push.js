self.addEventListener('install', () => {
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener('push', (event) => {
  if (!event.data) {
    return;
  }

  let payload;
  try {
    payload = event.data.json();
  } catch {
    payload = {
      title: 'Glory RS',
      body: event.data.text(),
      data: {},
    };
  }

  const title = payload.title || 'Glory RS';
  const options = {
    body: payload.body || '',
    icon: payload.icon || '/favicon.svg',
    badge: payload.badge || '/favicon.svg',
    data: payload.data || {},
    tag: payload.tag || 'general',
    renotify: true,
  };

  event.waitUntil(self.registration.showNotification(title, options));
});

self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  const rawLink = event.notification.data && event.notification.data.enlace
    ? event.notification.data.enlace
    : '/';
  const targetUrl = new URL(rawLink, self.location.origin).href;

  event.waitUntil(
    self.clients.matchAll({ type: 'window', includeUncontrolled: true }).then((clients) => {
      for (const client of clients) {
        if ('focus' in client) {
          client.postMessage({
            tipo: 'push-click',
            enlace: targetUrl,
          });
          return client.focus();
        }
      }
      return self.clients.openWindow(targetUrl);
    })
  );
});