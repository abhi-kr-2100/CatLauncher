import AboutPage from "@/pages/AboutPage";
import AchievementsPage from "@/pages/AchievementsPage";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";
import SettingsPage from "@/pages/SettingsPage";
import {
  Award,
  FileUp,
  Gamepad2,
  Info,
  Music,
  Settings,
} from "lucide-react";

export const routes = [
  {
    path: "/",
    element: <PlayPage />,
    label: "Play",
    icon: Gamepad2,
  },
  {
    path: "/achievements",
    element: <AchievementsPage />,
    label: "Achievements",
    icon: Award,
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
