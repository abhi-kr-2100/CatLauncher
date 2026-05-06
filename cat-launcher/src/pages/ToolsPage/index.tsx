import { useCallback, useState } from "react";
import { Navigate, Route, Routes } from "react-router-dom";

import ToolsSidebar from "./components/ToolsSidebar";
import { toolRoutes } from "./routes";

/**
 * The main page component for the Tools section.
 * Manages the layout with a sidebar and a content area for different utility views.
 *
 * @returns A React element representing the tools page.
 */
export default function ToolsPage() {
  const [isCollapsed, setIsCollapsed] = useState(true);

  const onToggleCollapse = useCallback(() => {
    setIsCollapsed((prev) => !prev);
  }, []);

  return (
    <div className="flex flex-1 overflow-hidden">
      <ToolsSidebar
        isCollapsed={isCollapsed}
        onToggleCollapse={onToggleCollapse}
      />

      <section className="flex-1 overflow-y-auto p-8">
        <Routes>
          <Route
            index
            element={<Navigate to={toolRoutes[0].path} replace />}
          />
          {toolRoutes.map((route) => (
            <Route
              key={route.path}
              path={route.path}
              element={route.element}
            />
          ))}
        </Routes>
      </section>
    </div>
  );
}
