import PlayPage from "@/pages/PlayPage";
import AboutPage from "@/pages/AboutPage";
import { Gamepad2, Info } from "lucide-react";

export const routes = [
    {
        path: "/",
        element: <PlayPage />,
        label: "Play",
        icon: Gamepad2,
    },
    {
        path: "/about",
        element: <AboutPage />,
        label: "About",
        icon: Info,
    },
];
