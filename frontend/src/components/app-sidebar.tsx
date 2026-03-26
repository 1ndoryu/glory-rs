/* [263A-16] Sidebar principal del restaurante.
 * Navegación adaptada de dashboard-01 con react-router-dom Links.
 * Collapsible "icon" para colapsar a solo iconos (tarea 22). */

import * as React from "react"
import { Link } from "react-router-dom"

import { NavMain } from "@/components/nav-main"
import { NavSecondary } from "@/components/nav-secondary"
import { NavUser } from "@/components/nav-user"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import {
  DollarSign,
  BarChart3,
  ClipboardList,
  Calendar,
  Users,
  Radio,
  UserX,
  Map,
  Settings,
  UtensilsCrossed,
  Megaphone,
  LayoutDashboard,
  MessageSquare,
  Bell,
} from "lucide-react"

const navPrincipal = [
  { title: "Dashboard", url: "/", icon: <LayoutDashboard /> },
  { title: "Ventas", url: "/ventas", icon: <DollarSign /> },
  { title: "Gastos", url: "/gastos", icon: <BarChart3 /> },
  { title: "Reservas", url: "/reservas", icon: <ClipboardList /> },
  { title: "Calendario", url: "/reservas/calendario", icon: <Calendar /> },
  { title: "Clientes", url: "/clientes", icon: <Users /> },
  { title: "Canales", url: "/canales", icon: <Radio /> },
  { title: "No-Shows", url: "/reservas/no-shows", icon: <UserX /> },
  { title: "Plano de Sala", url: "/plano-sala", icon: <Map /> },
  { title: "Campañas", url: "/marketing/campanas", icon: <Megaphone /> },
  { title: "Plantillas WA", url: "/marketing/plantillas", icon: <MessageSquare /> },
  { title: "Recordatorios", url: "/marketing/recordatorios", icon: <Bell /> },
]

const navSecundario = [
  { title: "Configuración", url: "/configuracion", icon: <Settings /> },
]

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:p-1.5!"
            >
              <Link to="/">
                <UtensilsCrossed className="size-5!" />
                <span className="text-base font-semibold">Restaurante</span>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={navPrincipal} />
        <NavSecondary items={navSecundario} className="mt-auto" />
      </SidebarContent>
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
    </Sidebar>
  )
}
