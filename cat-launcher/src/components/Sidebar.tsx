import { NavLink } from "react-router-dom";
import { PanelLeft } from "lucide-react";
import { ROUTES } from "@/lib/routes";
import { cn } from "@/lib/utils";

interface SidebarProps {
  isCollapsed: boolean;
  setIsCollapsed: (isCollapsed: boolean) => void;
}

const Sidebar = ({ isCollapsed, setIsCollapsed }: SidebarProps) => {
  const toggleSidebar = () => {
    setIsCollapsed(!isCollapsed);
  };

  const getLinkClass = (isActive: boolean, isDisabled: boolean) =>
    cn("flex items-center p-4", {
      "bg-gray-700": isActive,
      "hover:bg-gray-700": !isDisabled,
      "opacity-50 cursor-not-allowed": isDisabled,
    });

  return (
    <div
      className={`fixed top-0 left-0 h-full flex flex-col ${
        isCollapsed ? "w-20" : "w-64"
      } bg-gray-800 text-white transition-all duration-300`}
    >
      <div className="p-4 flex items-center justify-between">
        {!isCollapsed && (
          <h1 className="text-2xl font-bold">Cat Launcher</h1>
        )}
        <button onClick={toggleSidebar} className="p-2 hover:bg-gray-700 rounded">
          <PanelLeft />
        </button>
      </div>
      <nav className="flex-grow">
        <ul>
          {ROUTES.map((route) => (
            <li key={route.path}>
              <NavLink
                to={route.path}
                className={({ isActive }) =>
                  getLinkClass(isActive, route.isDisabled)
                }
                onClick={(e) => {
                  if (route.isDisabled) {
                    e.preventDefault();
                  }
                }}
              >
                <route.icon />
                {!isCollapsed && <span className="ml-4">{route.name}</span>}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
    </div>
  );
};

export default Sidebar;
