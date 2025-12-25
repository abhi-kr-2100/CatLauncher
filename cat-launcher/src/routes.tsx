import {
  Drum,
  FileUp,
  Gamepad2,
  Info,
  Package,
  Palette,
  Settings,
} from "lucide-react";

import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import PlayPage from "@/pages/PlayPage";
import SettingsPage from "@/pages/SettingsPage";
import SoundpacksPage from "@/pages/SoundpacksPage";
import TilesetsPage from "@/pages/TilesetsPage";

export const routes = [
  {
    path: "/",
    element: <PlayPage />,
    label: "Play",
    icon: Gamepad2,
  },
  {
    path: "/backups",
    element: <BackupsPage />,
    label: "Backups",
    icon: FileUp,
  },
  {
    path: "/mods",
    element: <ModsPage />,
    label: "Mods",
    icon: Package,
  },
  {
    path: "/soundpacks",
    element: <SoundpacksPage />,
    label: "Soundpacks",
    icon: Drum,
  },
  {
    path: "/tilesets",
    element: <TilesetsPage />,
    label: "Tilesets",
    icon: Palette,
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
