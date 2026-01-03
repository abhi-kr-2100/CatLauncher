import { useQuery } from "@tanstack/react-query";

import { SearchInput } from "@/components/SearchInput";
import type { GameVariant } from "@/generated-types/GameVariant";
import { listAllMods } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import ModCard from "./ModCard";
import { useEffect } from "react";
import { toastCL } from "@/lib/utils";
import { useSearch } from "@/hooks/useSearch";

interface ModsListProps {
  variant: GameVariant;
}

export default function ModsList({ variant }: ModsListProps) {
  const {
    data: mods,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.mods.listAll(variant),
    queryFn: () => listAllMods(variant),
  });

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: filteredMods,
    hasActiveSearch,
  } = useSearch(mods || [], {
    searchFn: (mod, query) => {
      return (
        mod.content.name.toLowerCase().includes(query) ||
        mod.content.id.toLowerCase().includes(query) ||
        mod.content.description.toLowerCase().includes(query)
      );
    },
  });

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load mods.", error);
    }
  }, [error]);

  if (isLoading) {
    return <p className="text-muted-foreground">Loading mods...</p>;
  }

  return (
    <div className="flex flex-col gap-4">
      <SearchInput
        value={searchQuery}
        onChange={setSearchQuery}
        placeholder="Search mods..."
        className="mb-4 mt-2"
      />
      {!filteredMods || filteredMods.length === 0 ? (
        <p className="text-muted-foreground">
          {hasActiveSearch
            ? "No mods match your search."
            : "No mods available."}
        </p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredMods.map((mod) => (
            <ModCard
              key={`${variant}-${mod.type}-${mod.content.id}`}
              variant={variant}
              mod={mod}
            />
          ))}
        </div>
      )}
    </div>
  );
}
