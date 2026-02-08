import type { BaseRoute } from "@/routes";
import { Clock, Trophy } from "lucide-react";

import PlayTime from "./components/PlayTime";
import GameAchievements from "./components/GameAchievements";

export type AchievementsRoute = BaseRoute;

export const achievementsRoutes: AchievementsRoute[] = [
  {
    path: "play-time",
    element: <PlayTime />,
    label: "Play Time",
    icon: Clock,
  },
  {
    path: "game-achievements",
    element: <GameAchievements />,
    label: "Game Achievements",
    icon: Trophy,
  },
];
