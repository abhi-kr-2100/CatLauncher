import type { BaseRoute } from "@/routes";
import { Clock, Trophy } from "lucide-react";

import PlayTime from "./components/PlayTime";
import GameAchievements from "./components/GameAchievements";

/**
 * Represents a route within the achievements section.
 */
export type AchievementsRoute = BaseRoute;

/**
 * The collection of routes available within the achievements page.
 * Each route includes a path, element, label, and icon for navigation.
 */
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
