import AboutPage from "@/pages/AboutPage";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";
import { Settings } from "@/pages/Settings";
import { FileUp, Gamepad2, Info, Music, Settings as SettingsIcon } from "lucide-react";

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
  {
    path: "/settings",
    element: <Settings />,
    label: "Settings",
    icon: SettingsIcon,
  }
];
