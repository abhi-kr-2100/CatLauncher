import AboutPage from "@/pages/AboutPage/index";
import AssetsPage from "@/pages/AssetsPage";
import BackupsPage from "@/pages/BackupsPage";
import PlayPage from "@/pages/PlayPage";
import { FileUp, Gamepad2, Info, Music } from "lucide-react";

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
];
