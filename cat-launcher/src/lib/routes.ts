import {
  Play,
  Archive,
  Puzzle,
  Grid,
  Music,
  Settings,
  Info,
  LucideProps,
} from "lucide-react";

import PlayPage from "@/PlayPage/PlayPage";
import AboutPage from "@/pages/AboutPage";
import SettingsPage from "@/pages/SettingsPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import TilesetsPage from "@/pages/TilesetsPage";
import SoundpacksPage from "@/pages/SoundpacksPage";
import { FC } from "react";

interface Route {
  path: string;
  name: string;
  icon: React.ComponentType<LucideProps>;
  component: FC;
  isDisabled: boolean;
}

export const ROUTES: Route[] = [
  {
    path: "/",
    name: "Play",
    icon: Play,
    component: PlayPage,
    isDisabled: false,
  },
  {
    path: "/backups",
    name: "Backups",
    icon: Archive,
    component: BackupsPage,
    isDisabled: true,
  },
  {
    path: "/mods",
    name: "Mods",
    icon: Puzzle,
    component: ModsPage,
    isDisabled: true,
  },
  {
    path: "/tilesets",
    name: "Tilesets",
    icon: Grid,
    component: TilesetsPage,
    isDisabled: true,
  },
  {
    path: "/soundpacks",
    name: "Soundpack",
    icon: Music,
    component: SoundpacksPage,
    isDisabled: true,
  },
  {
    path: "/settings",
    name: "Settings",
    icon: Settings,
    component: SettingsPage,
    isDisabled: true,
  },
  {
    path: "/about",
    name: "About",
    icon: Info,
    component: AboutPage,
    isDisabled: true,
  },
];
