import {
  Award,
  FileUp,
  Gamepad2,
  Info,
  Music,
  Settings,
  Wrench,
} from "lucide-react";

import type { JSX, ReactNode } from "react";
import type { LucideIcon } from "lucide-react";

import AboutPage from "@/pages/AboutPage";
import AchievementsPage from "@/pages/AchievementsPage";
import ToolsPage from "@/pages/ToolsPage";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";
import SettingsPage from "@/pages/SettingsPage";

export interface BaseRoute {
  path: string;
  element: ReactNode;
  label: string;
  icon: LucideIcon;
  customWrapper?: ({ children }: WrapperProps) => JSX.Element;
}

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
    customWrapper: SidenavCompatibleWrapper,
  },
  {
    path: "/tools/*",
    element: <ToolsPage />,
    label: "Tools",
    icon: Wrench,
    customWrapper: SidenavCompatibleWrapper,
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

interface WrapperProps {
  children: ReactNode;
}

export function DefaultWrapper({ children }: WrapperProps) {
  return (
    <div className="flex-1 overflow-y-auto">
      <div className="container mx-auto py-6">{children}</div>
    </div>
  );
}

export function SidenavCompatibleWrapper({ children }: WrapperProps) {
  return (
    <div className="flex flex-1 overflow-hidden">{children}</div>
  );
}
