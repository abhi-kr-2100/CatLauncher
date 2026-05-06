import {
  Award,
  FileUp,
  Gamepad2,
  Info,
  Music,
  Settings,
  Wrench,
} from "lucide-react";

import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";

import AboutPage from "@/pages/AboutPage";
import AchievementsPage from "@/pages/AchievementsPage";
import ToolsPage from "@/pages/ToolsPage";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";
import SettingsPage from "@/pages/SettingsPage";

/**
 * Defines the structure for a route in the application.
 */
export interface BaseRoute {
  /**
   * The URL path for the route.
   */
  path: string;
  /**
   * The React component to render for this route.
   */
  element: ReactNode;
  /**
   * The display label for the route, often used in navigation.
   */
  label: string;
  /**
   * The icon associated with the route.
   */
  icon: LucideIcon;
  /**
   * The layout type for this route. Defaults to "default".
   */
  layout?: "sidebar" | "default";
  /**
   * Whether the route should be hidden from navigation.
   */
  hidden?: boolean;
}

/**
 * The collection of primary routes for the application.
 */
export const routes: BaseRoute[] = [
  {
    path: "/",
    element: <PlayPage />,
    label: "Play",
    icon: Gamepad2,
  },
  {
    path: "/achievements/*",
    element: <AchievementsPage />,
    label: "Achievements",
    icon: Award,
    layout: "sidebar",
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
    path: "/settings",
    element: <SettingsPage />,
    label: "Settings",
    icon: Settings,
  },
  {
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
