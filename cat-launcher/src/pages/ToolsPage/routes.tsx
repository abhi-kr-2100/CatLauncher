import type { BaseRoute } from "@/routes";
import { Settings2, BookOpen, Download, Upload } from "lucide-react";

import WorldOptions from "./components/WorldOptions";
import Guide from "./components/Guide";
import ExportGameData from "./components/ExportGameData";
import ImportGameData from "./components/ImportGameData";

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
  {
    path: "export-game-data",
    element: <ExportGameData />,
    label: "Export Game Data",
    icon: Download,
  },
  {
    path: "import-game-data",
    element: <ImportGameData />,
    label: "Import Game Data",
    icon: Upload,
  },
];
