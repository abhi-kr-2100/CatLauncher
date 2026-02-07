import { Sidebar } from "@/components/Sidebar";
import { toolRoutes } from "../routes";

interface ToolsSidebarProps {
  isCollapsed: boolean;
  onToggleCollapse: () => void;
}

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
