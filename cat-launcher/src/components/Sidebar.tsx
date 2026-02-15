import { ChevronLeft, ChevronRight } from "lucide-react";
import { NavLink } from "react-router-dom";
import type { LucideIcon } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

export interface SidebarItem {
  path: string;
  label: string;
  icon: LucideIcon;
}

export interface SidebarProps {
  items: SidebarItem[];
  isCollapsed: boolean;
  onToggleCollapse: () => void;
  basePath?: string;
}

interface SidebarNavItemProps {
  item: SidebarItem;
  isCollapsed: boolean;
  basePath: string;
}

function SidebarNavItem({
  item,
  isCollapsed,
  basePath,
}: SidebarNavItemProps) {
  const Icon = item.icon;
  const fullPath = basePath
    ? `${basePath.endsWith("/") ? basePath : `${basePath}/`}${item.path}`
    : item.path;

  return (
    <NavLink
      to={fullPath}
      className={({ isActive }) =>
        cn(
          "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-all",
          isActive
            ? "bg-primary text-primary-foreground shadow-sm"
            : "text-muted-foreground hover:bg-accent hover:text-primary",
        )
      }
      title={isCollapsed ? item.label : undefined}
    >
      <Icon className="h-4 w-4 shrink-0" />
      <span
        className={cn(
          "transition-all duration-300 overflow-hidden whitespace-nowrap",
          isCollapsed ? "w-0 opacity-0" : "w-auto opacity-100",
        )}
      >
        {item.label}
      </span>
    </NavLink>
  );
}

export function Sidebar({
  items,
  isCollapsed,
  onToggleCollapse,
  basePath = "",
}: SidebarProps) {
  return (
    <aside
      className={cn(
        "relative flex flex-col border-r bg-muted/30 transition-all duration-300 min-h-full",
        isCollapsed ? "w-14" : "w-64",
      )}
    >
      <div className="flex items-center justify-center border-b">
        <Button
          variant="ghost"
          size="icon"
          className="w-full justify-center rounded-none"
          title={isCollapsed ? "Expand" : "Collapse"}
          onClick={onToggleCollapse}
        >
          {isCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </Button>
      </div>

      <nav className="flex-1 overflow-y-auto p-2">
        <div className="flex flex-col gap-2">
          {items.map((item) => (
            <SidebarNavItem
              key={item.path}
              item={item}
              isCollapsed={isCollapsed}
              basePath={basePath}
            />
          ))}
        </div>
      </nav>
    </aside>
  );
}
