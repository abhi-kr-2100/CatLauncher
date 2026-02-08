import { Sidebar } from "@/components/Sidebar";
import { achievementsRoutes } from "../routes";

interface AchievementsSidebarProps {
  isCollapsed: boolean;
  onToggleCollapse: () => void;
}

export default function AchievementsSidebar({
  isCollapsed,
  onToggleCollapse,
}: AchievementsSidebarProps) {
  return (
    <Sidebar
      items={achievementsRoutes}
      isCollapsed={isCollapsed}
      onToggleCollapse={onToggleCollapse}
      basePath="/achievements"
    />
  );
}
