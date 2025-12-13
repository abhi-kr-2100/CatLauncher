import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import TilesetCard from "./TilesetCard";
import { useListAllTilesets } from "./hooks";

interface TilesetsListProps {
  variant: GameVariant;
}

export default function TilesetsList({ variant }: TilesetsListProps) {
  const { tilesets, isLoading, error } = useListAllTilesets(variant);

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load tilesets.", error);
    }
  }, [error]);

  if (isLoading) {
    return (
      <p className="text-muted-foreground">Loading tilesets...</p>
    );
  }

  if (!tilesets || tilesets.length === 0) {
    return (
      <p className="text-muted-foreground">No tilesets available.</p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {tilesets.map((tileset) => (
        <TilesetCard
          key={`${variant}-${tileset.content.id}`}
          variant={variant}
          tileset={tileset}
        />
      ))}
    </div>
  );
}
