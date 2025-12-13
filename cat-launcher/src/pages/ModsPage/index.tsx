import { useState } from "react";

import { SearchInput } from "@/components/SearchInput";
import VariantSelector from "@/components/VariantSelector";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useSearch } from "@/hooks/useSearch";
import ModsList from "./ModsList";

function ModsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  const { searchInput, setSearchInput } = useSearch([], {
    debounceDelay: 300,
  });

  return (
    <div className="flex flex-col gap-2">
      <VariantSelector
        gameVariants={gameVariants}
        selectedVariant={selectedVariant}
        onVariantChange={setSelectedVariant}
        isLoading={gameVariantsLoading}
      />
      {selectedVariant && (
        <>
          <SearchInput
            value={searchInput}
            onChange={setSearchInput}
            placeholder="Search mods..."
            className="mb-4 mt-2"
          />
          <ModsList
            variant={selectedVariant}
            searchInput={searchInput}
            setSearchInput={setSearchInput}
          />
        </>
      )}
    </div>
  );
}

export default ModsPage;
