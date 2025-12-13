import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import PlayPage from "@/pages/PlayPage";
import TilesetsPage from "@/pages/TilesetsPage";
import { FileUp, Gamepad2, Info, Package, Palette, Drum } from "lucide-react";
import SoundpacksPage from "@/pages/SoundpacksPage";

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
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
