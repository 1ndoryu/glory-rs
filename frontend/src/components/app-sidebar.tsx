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
  Home,
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
} from "lucide-react"

const navPrincipal = [
  { title: "Inicio", url: "/", icon: <Home /> },
  { title: "Ventas", url: "/ventas", icon: <DollarSign /> },
  { title: "Gastos", url: "/gastos", icon: <BarChart3 /> },
  { title: "Reservas", url: "/reservas", icon: <ClipboardList /> },
  { title: "Calendario", url: "/reservas/calendario", icon: <Calendar /> },
  { title: "Dashboard", url: "/reservas/dashboard", icon: <LayoutDashboard /> },
  { title: "Clientes", url: "/clientes", icon: <Users /> },
  { title: "Canales", url: "/canales", icon: <Radio /> },
  { title: "No-Shows", url: "/reservas/no-shows", icon: <UserX /> },
  { title: "Plano de Sala", url: "/plano-sala", icon: <Map /> },
  { title: "Campañas", url: "/marketing/campanas", icon: <Megaphone /> },
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
