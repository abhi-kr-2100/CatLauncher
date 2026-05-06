import { useEffect } from "react";

import { SearchInput } from "@/components/SearchInput";
import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import SoundpackCard from "./SoundpackCard";
import { useListAllSoundpacks } from "./hooks";
import { useSearch } from "@/hooks/useSearch";

/**
 * Props for the SoundpacksList component.
 */
interface SoundpacksListProps {
  /**
   * The game variant for which to display soundpacks.
   */
  variant: GameVariant;
}

/**
 * The SoundpacksList component renders a searchable grid of soundpacks available
 * for a specific game variant. It handles fetching the soundpacks, filtering
 * based on user input, and displaying loading or empty states.
 *
 * @param props - The component props.
 * @returns A React component that renders the list of soundpacks and a search input.
 */
export default function SoundpacksList({
  variant,
}: SoundpacksListProps) {
  const { soundpacks, isLoading, error } =
    useListAllSoundpacks(variant);

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: filteredSoundpacks,
    hasActiveSearch,
  } = useSearch(soundpacks || [], {
    searchFn: (soundpack, query) => {
      return (
        soundpack.content.name.toLowerCase().includes(query) ||
        soundpack.content.id.toLowerCase().includes(query)
      );
    },
  });

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load soundpacks.", error);
    }
  }, [error]);

  if (isLoading) {
    return (
      <p className="text-muted-foreground">Loading soundpacks...</p>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <SearchInput
        value={searchQuery}
        onChange={setSearchQuery}
        placeholder="Search soundpacks..."
        className="mb-4 mt-2"
      />
      {!filteredSoundpacks || filteredSoundpacks.length === 0 ? (
        <p className="text-muted-foreground">
          {hasActiveSearch
            ? "No soundpacks match your search."
            : "No soundpacks available."}
        </p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredSoundpacks.map((soundpack) => (
            <SoundpackCard
              key={`${variant}-${soundpack.content.id}`}
              variant={variant}
              soundpack={soundpack}
            />
          ))}
        </div>
      )}
    </div>
  );
}
