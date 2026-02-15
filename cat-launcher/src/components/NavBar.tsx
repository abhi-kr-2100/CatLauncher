import { NavLink } from "react-router-dom";

import ThemeToggle from "@/theme/ThemeToggle";
import { cn } from "@/lib/utils";
import { BaseRoute, routes } from "@/routes";

interface NavItemProps {
  route: BaseRoute;
}

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

export default function NavBar() {
  return (
    <nav className="sticky top-0 z-50 flex shrink-0 items-center justify-between gap-4 border-b bg-background px-4 py-3">
      <div className="flex flex-1 items-center justify-center gap-4">
        {routes.map((route) => (
          <NavItem key={route.path} route={route} />
        ))}
      </div>
      <ThemeToggle />
    </nav>
  );
}
