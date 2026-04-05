/* [044A-38 Fase 9] Campana de notificaciones con dropdown.
 * Badge de no leídas, dropdown con lista, marcar leídas, navegar a link. */

import { useCallback, useState } from 'react';
import { Bell, Check, CheckCheck } from 'lucide-react';

import { NOTIF_TYPES, type NotificationType } from '../../api/notifications';
import { useNotifications, useNotificationWs } from '../../hooks/useNotifications';
import { Button } from '../ui/Button';
import { MenuContextual } from '../ui/ContextMenu';
import './NotificationBell.css';

export default function NotificationBell() {
  const {
    notifications,
    unreadCount,
    marcarLeidas,
    marcarTodasLeidas,
  } = useNotifications();

  /* Activar WebSocket push */
  useNotificationWs();

  const [open, setOpen] = useState(false);

  const toggleDropdown = useCallback(() => {
    setOpen((prev) => !prev);
  }, []);

  const closeDropdown = useCallback(() => {
    setOpen(false);
  }, []);

  const handleMarkRead = useCallback(
    (id: string) => {
      marcarLeidas([id]);
    },
    [marcarLeidas]
  );

  const handleMarkAll = useCallback(() => {
    marcarTodasLeidas();
  }, [marcarTodasLeidas]);

  /* Formatear fecha relativa simple */
  const formatTime = (iso: string): string => {
    const diff = Date.now() - new Date(iso).getTime();
    const mins = Math.floor(diff / 60000);
    if (mins < 1) return 'ahora';
    if (mins < 60) return `hace ${mins}m`;
    const hours = Math.floor(mins / 60);
    if (hours < 24) return `hace ${hours}h`;
    const days = Math.floor(hours / 24);
    return `hace ${days}d`;
  };

  return (
    <MenuContextual
      abierto={open}
      onToggle={toggleDropdown}
      onCerrar={closeDropdown}
      ariaLabel={`Notificaciones${unreadCount > 0 ? ` (${unreadCount} sin leer)` : ''}`}
      className="notificationBell"
      triggerClassName="notificationBell__trigger"
      panelClassName="notificationBell__dropdown"
      triggerContent={(
        <>
          <Bell size={20} />
          {unreadCount > 0 && (
            <span className="notificationBell__badge">
              {unreadCount > 99 ? '99+' : unreadCount}
            </span>
          )}
        </>
      )}
    >
      <div className="notificationBell__header">
        <h3 className="notificationBell__title">Notificaciones</h3>
        {unreadCount > 0 && (
          <Button
            className="notificationBell__markAll"
            onClick={handleMarkAll}
            title="Marcar todas como leídas"
            type="button"
            variante="texto"
            tamano="pequeno"
          >
            <CheckCheck size={16} />
          </Button>
        )}
      </div>

      <div className="notificationBell__list">
        {notifications.length === 0 ? (
          <p className="notificationBell__empty">Sin notificaciones</p>
        ) : (
          notifications.map((n) => {
            const typeInfo = NOTIF_TYPES[n.notification_type as NotificationType];
            return (
              <div
                key={n.id}
                className={`notificationBell__item ${!n.read ? 'notificationBell__item--unread' : ''}`}
              >
                <div className="notificationBell__itemContent">
                  <span className="notificationBell__itemTitle">
                    {typeInfo?.label ?? n.notification_type}: {n.title}
                  </span>
                  {n.body && (
                    <span className="notificationBell__itemBody">{n.body}</span>
                  )}
                  <span className="notificationBell__itemTime">
                    {formatTime(n.created_at)}
                  </span>
                </div>
                {!n.read && (
                  <Button
                    className="notificationBell__markRead"
                    onClick={() => handleMarkRead(n.id)}
                    title="Marcar como leída"
                    type="button"
                    variante="texto"
                    tamano="pequeno"
                  >
                    <Check size={14} />
                  </Button>
                )}
              </div>
            );
          })
        )}
      </div>
    </MenuContextual>
  );
}
