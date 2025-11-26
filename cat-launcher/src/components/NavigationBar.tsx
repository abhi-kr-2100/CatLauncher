import { NavLink } from "react-router-dom";
import { Home, Info } from "lucide-react";

function NavigationBar() {
  return (
    <nav className="flex items-center gap-2 p-2 bg-background-inset">
      <NavLink
        to="/"
        className={({ isActive }) =>
          `flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
            isActive
              ? "bg-primary-hover text-primary-foreground"
              : "text-muted-foreground hover:bg-primary-hover/50"
          }`
        }
      >
        <Home className="h-4 w-4" />
        Play
      </NavLink>
      <NavLink
        to="/about"
        className={({ isActive }) =>
          `flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors ${
            isActive
              ? "bg-primary-hover text-primary-foreground"
              : "text-muted-foreground hover:bg-primary-hover/50"
          }`
        }
      >
        <Info className="h-4 w-4" />
        About
      </NavLink>
    </nav>
  );
}

export default NavigationBar;
