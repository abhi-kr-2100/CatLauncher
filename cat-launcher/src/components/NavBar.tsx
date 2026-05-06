import { NavLink } from "react-router-dom";

import ThemeToggle from "@/theme/ThemeToggle";
import { cn } from "@/lib/utils";
import { BaseRoute, routes } from "@/routes";

/**
 * Props for the {@link NavItem} component.
 */
interface NavItemProps {
  /**
   * The route object to render.
   */
  route: BaseRoute;
}

/**
 * A single navigation item in the navigation bar.
 *
 * @param props - The component props.
 * @returns A React component that renders a navigation link.
 */
function NavItem({ route }: NavItemProps) {
  const Icon = route.icon;
  const targetPath = route.path.replace("/*", "");

  return (
    <NavLink
      to={targetPath}
      end={targetPath === "/"}
      className={({ isActive }) =>
        cn(
          "flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-all",
          isActive
            ? "bg-primary text-primary-foreground shadow-sm"
            : "text-muted-foreground hover:bg-accent hover:text-primary",
        )
      }
    >
      {Icon && <Icon className="h-4 w-4" />}
      {route.label}
    </NavLink>
  );
}

/**
 * The main navigation bar component, displaying links to visible routes and a theme toggle.
 *
 * @returns A React component that renders the navigation bar.
 */
export default function NavBar() {
  const visibleRoutes = routes.filter((route) => !route.hidden);

  return (
    <nav className="flex shrink-0 items-center justify-between gap-4 border-b bg-background px-4 py-3">
      <div className="flex flex-1 items-center justify-center gap-4">
        {visibleRoutes.map((route) => (
          <NavItem key={route.path} route={route} />
        ))}
      </div>
      <ThemeToggle />
    </nav>
  );
}
