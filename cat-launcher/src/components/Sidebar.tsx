import { ChevronLeft, ChevronRight } from "lucide-react";
import { NavLink } from "react-router-dom";
import type { LucideIcon } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

/**
 * Represents an individual item within the sidebar navigation.
 *
 * @public
 */
export interface SidebarItem {
  /** The destination path for the navigation link. */
  path: string;
  /** The display label for the item. */
  label: string;
  /** The icon to display alongside the label. */
  icon: LucideIcon;
  /** Whether the item should be hidden from the sidebar. */
  hidden?: boolean;
}

/**
 * Properties for the {@link Sidebar} component.
 *
 * @public
 */
export interface SidebarProps {
  /** The list of navigation items to display. */
  items: SidebarItem[];
  /** Whether the sidebar is currently in a collapsed state. */
  isCollapsed: boolean;
  /** Callback function triggered when the collapse state is toggled. */
  onToggleCollapse: () => void;
  /** An optional base path to prepend to all item paths. */
  basePath?: string;
}

/**
 * Properties for the internal {@link SidebarNavItem} component.
 *
 * @internal
 */
interface SidebarNavItemProps {
  /** The sidebar item to render. */
  item: SidebarItem;
  /** Whether the sidebar is collapsed. */
  isCollapsed: boolean;
  /** The base path for navigation. */
  basePath: string;
}

/**
 * An individual navigation link component used within the sidebar.
 *
 * @param props - The properties for the sidebar navigation item.
 * @returns A React element representing a single navigation link.
 *
 * @internal
 */
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

/**
 * The main sidebar navigation component.
 * Provides a collapsible menu with icons and labels.
 *
 * @param props - The properties for the sidebar.
 * @returns A React element representing the sidebar.
 *
 * @public
 */
export function Sidebar({
  items,
  isCollapsed,
  onToggleCollapse,
  basePath = "",
}: SidebarProps) {
  return (
    <aside
      className={cn(
        "relative flex flex-col border-r bg-muted/30 transition-all duration-300",
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
          {items
            .filter((item) => !item.hidden)
            .map((item) => (
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
