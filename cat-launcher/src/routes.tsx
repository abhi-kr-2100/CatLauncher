import PlayPage from "@/pages/PlayPage";
import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import { FileUp, Gamepad2, Info, Puzzle } from "lucide-react";

export const routes = [
  {
    path: "/",
    element: <PlayPage />,
    label: "Play",
    icon: Gamepad2,
  },
  {
    path: "/mods",
    element: <ModsPage />,
    label: "Mods",
    icon: Puzzle,
  },
  {
    path: "/backups",
    element: <BackupsPage />,
    label: "Backups",
    icon: FileUp,
  },
  {
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
