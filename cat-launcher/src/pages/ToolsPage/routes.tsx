import type { BaseRoute } from "@/routes";
import { Settings2, BookOpen } from "lucide-react";

import WorldOptions from "./WorldOptions";
import Guide from "./components/Guide";

export type ToolRoute = BaseRoute;

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
