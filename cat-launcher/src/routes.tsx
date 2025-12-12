import { FileUp, Gamepad2, Info, Box } from "lucide-react";

import PlayPage from "@/pages/PlayPage";
import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";

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
    icon: Box,
  },
  {
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
