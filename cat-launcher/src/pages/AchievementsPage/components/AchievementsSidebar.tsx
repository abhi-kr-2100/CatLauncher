import { Sidebar } from "@/components/Sidebar";
import { achievementsRoutes } from "../routes";

/**
 * Props for the {@link AchievementsSidebar} component.
 */
interface AchievementsSidebarProps {
  /**
   * Whether the sidebar is currently collapsed.
   */
  isCollapsed: boolean;
  /**
   * Callback function to toggle the sidebar collapse state.
   */
  onToggleCollapse: () => void;
}

/**
 * A sidebar component specifically for the Achievements page navigation.
 *
 * @param props - The component props.
 * @returns A React element representing the achievements sidebar.
 */
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
