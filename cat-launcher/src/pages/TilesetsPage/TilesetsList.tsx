import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import TilesetCard from "./TilesetCard";
import { useListAllTilesets } from "./hooks";
import { useSearch } from "@/hooks/useSearch";

interface TilesetsListProps {
  variant: GameVariant;
  searchInput: string;
  setSearchInput: (value: string) => void;
}

export default function TilesetsList({
  variant,
  searchInput,
  setSearchInput,
}: TilesetsListProps) {
  const { tilesets, isLoading, error } = useListAllTilesets(variant);

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load tilesets.", error);
    }
  }, [error]);

  const { filteredItems: filteredTilesets, hasActiveSearch } =
    useSearch(tilesets || [], {
      searchInput,
      setSearchInput,
      searchFn: (tileset, query) => {
        const lowerQuery = query.toLowerCase().trim();
        return (
          tileset.content.name.toLowerCase().includes(lowerQuery) ||
          tileset.content.id.toLowerCase().includes(lowerQuery)
        );
      },
    });

  if (isLoading) {
    return (
      <p className="text-muted-foreground">Loading tilesets...</p>
    );
  }

  if (!filteredTilesets || filteredTilesets.length === 0) {
    return (
      <p className="text-muted-foreground">
        {hasActiveSearch
          ? "No tilesets match your search."
          : "No tilesets available."}
      </p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {filteredTilesets.map((tileset) => (
        <TilesetCard
          key={`${variant}-${tileset.content.id}`}
          variant={variant}
          tileset={tileset}
        />
      ))}
    </div>
  );
}
