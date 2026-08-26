import { FileUp, Gamepad2, Info, Music, Wrench } from "lucide-react";

import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";

import AboutPage from "@/pages/AboutPage";
import ToolsPage from "@/pages/ToolsPage";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";

export interface BaseRoute {
  path: string;

  element: ReactNode;

  label: string;

  icon: LucideIcon;

  layout?: "sidebar" | "default";

  hidden?: boolean;
}

export const routes: BaseRoute[] = [
  {
    path: "/",
    element: <PlayPage />,
    label: "Play",
    icon: Gamepad2,
  },
  {
    path: "/tools/*",
    element: <ToolsPage />,
    label: "Tools",
    icon: Wrench,
    layout: "sidebar",
    hidden: true,
  },
  {
    path: "/backups",
    element: <BackupsPage />,
    label: "Backups",
    icon: FileUp,
  },
  {
    path: "/assets",
    element: <AssetsPage />,
    label: "Mods, Music & Tiles",
    icon: Music,
  },
  {
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
