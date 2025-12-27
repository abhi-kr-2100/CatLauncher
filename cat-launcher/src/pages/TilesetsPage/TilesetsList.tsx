import { useEffect } from "react";

import { SearchInput } from "@/components/SearchInput";
import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import TilesetCard from "./TilesetCard";
import { useListAllTilesets } from "./hooks";
import { useSearch } from "@/hooks/useSearch";
import { tilesetsPageErrorMap } from "./lib/errors";

interface TilesetsListProps {
  variant: GameVariant;
}

export default function TilesetsList({ variant }: TilesetsListProps) {
  const { tilesets, isLoading, error } = useListAllTilesets(variant);

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: filteredTilesets,
    hasActiveSearch,
  } = useSearch(tilesets || [], {
    searchFn: (tileset, query) => {
      return (
        tileset.content.name.toLowerCase().includes(query) ||
        tileset.content.id.toLowerCase().includes(query)
      );
    },
  });

  useEffect(() => {
    if (error) {
      toastCL(
        "error",
        "Failed to load tilesets.",
        error,
        tilesetsPageErrorMap,
      );
    }
  }, [error]);

  if (isLoading) {
    return (
      <p className="text-muted-foreground">Loading tilesets...</p>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <SearchInput
        value={searchQuery}
        onChange={setSearchQuery}
        placeholder="Search tilesets..."
        className="mb-4 mt-2"
      />
      {!filteredTilesets || filteredTilesets.length === 0 ? (
        <p className="text-muted-foreground">
          {hasActiveSearch
            ? "No tilesets match your search."
            : "No tilesets available."}
        </p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredTilesets.map((tileset) => (
            <TilesetCard
              key={`${variant}-${tileset.content.id}`}
              variant={variant}
              tileset={tileset}
            />
          ))}
        </div>
      )}
    </div>
  );
}
