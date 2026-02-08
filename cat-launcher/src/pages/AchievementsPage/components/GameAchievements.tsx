import { useState, useCallback, useMemo } from "react";

import { SearchInput } from "@/components/SearchInput";
import VariantSelector from "@/components/VariantSelector";
import type { CharacterAchievements } from "@/generated-types/CharacterAchievements";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useSearch } from "@/hooks/useSearch";
import { toastCL } from "@/lib/utils";
import AchievementsList from "./AchievementsList";
import { useAchievements } from "../hooks/useAchievements";

export default function GameAchievements() {
  const {
    gameVariants,
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useGameVariants();

  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  const onAchievementsError = useCallback((error: Error) => {
    toastCL("error", "Failed to load achievements", error);
  }, []);

  const { data: achievements, isLoading: achievementsLoading } =
    useAchievements(selectedVariant, onAchievementsError);

  const searchOptions = useMemo(
    () => ({
      searchFn: (item: CharacterAchievements, query: string) =>
        item.characterName.toLowerCase().includes(query),
    }),
    [],
  );

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: filteredAchievements,
  } = useSearch(achievements ?? [], searchOptions);

  if (gameVariantsLoading) {
    return <p>Loading...</p>;
  }

  if (gameVariantsError) {
    return (
      <p>Error: {gameVariantsErrorObj?.message ?? "Unknown error"}</p>
    );
  }

  return (
    <div className="flex flex-col space-y-4">
      <div className="flex gap-4 items-end">
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={setSelectedVariant}
          isLoading={gameVariantsLoading}
        />
        <SearchInput
          placeholder="Search character..."
          value={searchQuery}
          onChange={setSearchQuery}
        />
      </div>

      {achievementsLoading ? (
        <p>Loading achievements...</p>
      ) : (
        <AchievementsList achievements={filteredAchievements} />
      )}
    </div>
  );
}
