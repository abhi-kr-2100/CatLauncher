import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import SoundpackCard from "./SoundpackCard";
import { useListAllSoundpacks } from "./hooks";
import { useSearch } from "@/hooks/useSearch";

interface SoundpacksListProps {
  variant: GameVariant;
  searchInput: string;
  setSearchInput: (value: string) => void;
}

export default function SoundpacksList({
  variant,
  searchInput,
  setSearchInput,
}: SoundpacksListProps) {
  const { soundpacks, isLoading, error } =
    useListAllSoundpacks(variant);

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load soundpacks.", error);
    }
  }, [error]);

  const { filteredItems: filteredSoundpacks, hasActiveSearch } =
    useSearch(soundpacks || [], {
      searchInput,
      setSearchInput,
      searchFn: (soundpack, query) => {
        const lowerQuery = query.toLowerCase().trim();
        return (
          soundpack.content.name.toLowerCase().includes(lowerQuery) ||
          soundpack.content.id.toLowerCase().includes(lowerQuery)
        );
      },
    });

  if (isLoading) {
    return (
      <p className="text-muted-foreground">Loading soundpacks...</p>
    );
  }

  if (!filteredSoundpacks || filteredSoundpacks.length === 0) {
    return (
      <p className="text-muted-foreground">
        {hasActiveSearch
          ? "No soundpacks match your search."
          : "No soundpacks available."}
      </p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {filteredSoundpacks.map((soundpack) => (
        <SoundpackCard
          key={`${variant}-${soundpack.content.id}`}
          variant={variant}
          soundpack={soundpack}
        />
      ))}
    </div>
  );
}
