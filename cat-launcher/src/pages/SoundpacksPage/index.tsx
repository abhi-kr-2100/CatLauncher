import { useState } from "react";

import { SearchInput } from "@/components/SearchInput";
import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useSearch } from "@/hooks/useSearch";
import SoundpacksList from "./SoundpacksList";

function SoundpacksPage() {
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
            placeholder="Search soundpacks..."
            className="mb-4 mt-2"
          />
          <SoundpacksList
            variant={selectedVariant}
            searchInput={searchInput}
            setSearchInput={setSearchInput}
          />
        </>
      )}
    </div>
  );
}

export default SoundpacksPage;
