import { useCallback, useState } from "react";
import { Navigate, Route, Routes } from "react-router-dom";

import AchievementsSidebar from "./components/AchievementsSidebar";
import { achievementsRoutes } from "./routes";

/**
 * The main page component for the Achievements section.
 * Manages the layout with a sidebar and a content area for different achievement-related views.
 *
 * @returns A React element representing the achievements page.
 */
function AchievementsPage() {
  const [isCollapsed, setIsCollapsed] = useState(true);

  const onToggleCollapse = useCallback(() => {
    setIsCollapsed((prev) => !prev);
  }, []);

  return (
    <div className="flex flex-1 overflow-hidden">
      <AchievementsSidebar
        isCollapsed={isCollapsed}
        onToggleCollapse={onToggleCollapse}
      />

      <section className="flex-1 overflow-y-auto p-8">
        <Routes>
          <Route
            index
            element={
              <Navigate to={achievementsRoutes[0].path} replace />
            }
          />
          {achievementsRoutes.map((route) => (
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

export default AchievementsPage;
