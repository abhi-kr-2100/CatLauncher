import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import ModsPage from "@/pages/ModsPage";
import PlayPage from "@/pages/PlayPage";
import { FileUp, Gamepad2, Info, Package } from "lucide-react";

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
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
