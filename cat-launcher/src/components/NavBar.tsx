import { Link, useLocation } from "react-router-dom";

import ThemeToggle from "@/theme/ThemeToggle";
import { cn } from "@/lib/utils";
import { routes } from "@/routes";

export default function NavBar() {
  const location = useLocation();

  return (
    <nav className="flex shrink-0 items-center justify-between gap-4 border-b bg-background px-4 py-3">
      <div className="flex flex-1 items-center justify-center gap-4">
        {routes.map((route) => {
          const Icon = route.icon;
          return (
            <Link
              key={route.path}
              to={route.path}
              className={cn(
                "flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-all",
                location.pathname === route.path
                  ? "bg-primary text-primary-foreground shadow-sm"
                  : "text-muted-foreground hover:bg-accent hover:text-primary",
              )}
            >
              {Icon && <Icon className="h-4 w-4" />}
              {route.label}
            </Link>
          );
        })}
      </div>
      <ThemeToggle />
    </nav>
  );
}
