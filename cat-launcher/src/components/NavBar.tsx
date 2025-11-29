import { Link, useLocation } from "react-router-dom";
import { cn } from "@/lib/utils";
import { routes } from "@/routes";

export default function NavBar() {
  const location = useLocation();

  return (
    <nav className="flex items-center justify-center gap-6 p-4 border-b">
      {routes.map((route) => {
        const Icon = route.icon;
        return (
          <Link
            key={route.path}
            to={route.path}
            className={cn(
              "flex items-center gap-2 text-sm font-medium transition-all rounded-md px-3 py-2",
              location.pathname === route.path
                ? "bg-primary text-primary-foreground shadow-sm"
                : "text-muted-foreground hover:text-primary hover:bg-accent"
            )}
          >
            {Icon && <Icon className="h-4 w-4" />}
            {route.label}
          </Link>
        );
      })}
    </nav>
  );
}
