import { useState, useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useSearch } from "@/hooks/useSearch";
import { queryKeys } from "@/lib/queryKeys";
import { listAllMods } from "@/lib/commands";
import { toastCL } from "@/lib/utils";
import VariantSelector from "@/components/VariantSelector";
import { SearchInput } from "@/components/SearchInput";
import ModList from "./ModList";
import LoadingState from "./LoadingState";
import EmptyState from "./EmptyState";

function ModsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(
    null,
  );

  const activeVariant = selectedVariant ?? gameVariants[0]?.id;

  const {
    data: mods,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.mods(activeVariant),
    queryFn: () => listAllMods(activeVariant),
    enabled: !!activeVariant,
  });

  const {
    searchInput,
    setSearchInput,
    debouncedSearchQuery,
    filteredItems: filteredMods,
    hasActiveSearch,
  } = useSearch(mods ?? []);

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load mods", error);
    }
  }, [error]);

  // Early return if no active variant is available
  if (!activeVariant) {
    return <LoadingState />;
  }

  if (isLoading) {
    return <LoadingState />;
  }

  if (!mods || mods.length === 0) {
    return <EmptyState />;
  }

  if (hasActiveSearch && filteredMods.length === 0) {
    return (
      <div className="container mx-auto py-6">
        <div className="flex items-center gap-4 mb-6">
          <VariantSelector
            gameVariants={gameVariants}
            selectedVariant={selectedVariant}
            onVariantChange={setSelectedVariant}
            isLoading={gameVariantsLoading}
          />
          <SearchInput
            value={searchInput}
            onChange={setSearchInput}
            placeholder="Search mods..."
            className="flex-1 max-w-md"
          />
        </div>
        <h1 className="text-3xl font-bold mb-6">Available Mods</h1>
        <div className="text-center py-8">
          <p className="text-muted-foreground">
            No mods found matching "{debouncedSearchQuery}". Try a different
            search term.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2 p-2">
      <div className="flex items-center gap-4">
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={setSelectedVariant}
          isLoading={gameVariantsLoading}
        />
        <SearchInput
          value={searchInput}
          onChange={setSearchInput}
          placeholder="Search mods..."
          className="flex-1 max-w-md"
        />
      </div>

      <ModList mods={filteredMods} variant={activeVariant} />
    </div>
  );
}

export default ModsPage;
