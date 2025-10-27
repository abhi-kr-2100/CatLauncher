import {
  Play,
  Archive,
  Puzzle,
  Grid,
  Music,
  Settings,
  Info,
} from "lucide-react";

import PlayPage from "@/PlayPage/PlayPage";
import AboutPage from "@/pages/AboutPage";
import SettingsPage from "@/pages/SettingsPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import TilesetsPage from "@/pages/TilesetsPage";
import SoundpacksPage from "@/pages/SoundpacksPage";

export const ROUTES = [
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
