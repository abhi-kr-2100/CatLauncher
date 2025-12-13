import { useQuery } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { listAllMods } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import ModCard from "./ModCard";
import { useEffect } from "react";
import { toastCL } from "@/lib/utils";
import { useSearch } from "@/hooks/useSearch";

interface ModsListProps {
  variant: GameVariant;
  searchInput: string;
  setSearchInput: (value: string) => void;
}

export default function ModsList({
  variant,
  searchInput,
  setSearchInput,
}: ModsListProps) {
  const {
    data: mods,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.mods.listAll(variant),
    queryFn: () => listAllMods(variant),
  });

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load mods.", error);
    }
  }, [error]);

  const { filteredItems: filteredMods, hasActiveSearch } = useSearch(
    mods || [],
    {
      searchInput,
      setSearchInput,
      searchFn: (mod, query) => {
        const lowerQuery = query.toLowerCase().trim();
        return (
          mod.content.name.toLowerCase().includes(lowerQuery) ||
          mod.content.id.toLowerCase().includes(lowerQuery) ||
          mod.content.description?.toLowerCase().includes(lowerQuery)
        );
      },
    },
  );

  if (isLoading) {
    return <p className="text-muted-foreground">Loading mods...</p>;
  }

  if (!filteredMods || filteredMods.length === 0) {
    return (
      <p className="text-muted-foreground">
        {hasActiveSearch
          ? "No mods match your search."
          : "No mods available."}
      </p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {filteredMods.map((mod) => (
        <ModCard
          key={`${variant}-${mod.type}-${mod.content.id}`}
          variant={variant}
          mod={mod}
        />
      ))}
    </div>
  );
}
