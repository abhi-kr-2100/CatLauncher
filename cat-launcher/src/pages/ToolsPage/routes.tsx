import type { BaseRoute } from "@/routes";
import { Settings2, BookOpen } from "lucide-react";

import WorldOptions from "./components/WorldOptions";
import Guide from "./components/Guide";

/**
 * Represents a route within the tools section.
 */
export type ToolRoute = BaseRoute;

/**
 * The collection of routes available within the tools page.
 * Each route includes a path, element, label, and icon for navigation.
 */
export const toolRoutes: ToolRoute[] = [
  {
    path: "world-options",
    element: <WorldOptions />,
    label: "World Options",
    icon: Settings2,
  },
  {
    path: "guide",
    element: <Guide />,
    label: "Guide",
    icon: BookOpen,
  },
];
