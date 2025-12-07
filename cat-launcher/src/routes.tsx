import PlayPage from "@/pages/PlayPage";
import AboutPage from "@/pages/AboutPage";
import BackupsPage from "@/pages/BackupsPage";
import { FileUp, Gamepad2, Info } from "lucide-react";

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
    path: "/about",
    element: <AboutPage />,
    label: "About",
    icon: Info,
  },
];
