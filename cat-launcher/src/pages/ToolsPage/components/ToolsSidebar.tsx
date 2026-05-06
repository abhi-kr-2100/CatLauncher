import { Sidebar } from "@/components/Sidebar";
import { toolRoutes } from "../routes";

/**
 * Props for the {@link ToolsSidebar} component.
 */
interface ToolsSidebarProps {
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
 * A sidebar component specifically for the Tools page navigation.
 *
 * @param props - The component props.
 * @returns A React element representing the tools sidebar.
 */
export default function ToolsSidebar({
  isCollapsed,
  onToggleCollapse,
}: ToolsSidebarProps) {
  return (
    <Sidebar
      items={toolRoutes}
      isCollapsed={isCollapsed}
      onToggleCollapse={onToggleCollapse}
      basePath="/tools"
    />
  );
}
