import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { CharacterAchievements } from "@/generated-types/CharacterAchievements";

/**
 * Props for the {@link AchievementsList} component.
 */
interface AchievementsListProps {
  /**
   * An array of achievement data grouped by character.
   */
  achievements: CharacterAchievements[];
}

/**
 * A component that renders a list of achievements for multiple characters.
 * Each character's achievements are displayed in a card.
 *
 * @param props - The component props.
 * @returns A React element representing the list of achievements.
 */
export default function AchievementsList({
  achievements,
}: AchievementsListProps) {
  if (achievements.length === 0) {
    return (
      <div className="text-center text-muted-foreground p-8">
        No achievements found.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {achievements.map((charData) => (
        <Card key={charData.characterName}>
          <CardHeader className="pb-2">
            <CardTitle>{charData.characterName}</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-2">
              {charData.achievements.map((achievement) => (
                <Badge key={achievement.id} variant="secondary">
                  {achievement.name}
                </Badge>
              ))}
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
