import { SearchInput } from "@/components/SearchInput";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getVariantLabel, toastCL } from "@/lib/utils";
import { useSearch } from "@/hooks/useSearch";
import { useMods } from "./hooks";
import ModCard from "./ModCard";

interface ModsListProps {
  variant: GameVariant;
}

export default function ModsList({ variant }: ModsListProps) {
  const variantLabel = getVariantLabel(variant);
  const { mods, isLoading } = useMods(
    variant,
    (err) => toastCL("error", "Failed to load mods.", err),
    (err) =>
      toastCL(
        "error",
        `Failed to fetch mods for ${variantLabel}.`,
        err,
      ),
  );

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: filteredMods,
    hasActiveSearch,
  } = useSearch(mods, {
    searchFn: (mod, query) => {
      return (
        mod.content.name.toLowerCase().includes(query) ||
        mod.content.id.toLowerCase().includes(query) ||
        mod.content.description.toLowerCase().includes(query)
      );
    },
  });

  // Mods arrive in chunks; show loading message until first chunk arrives.
  if (isLoading && mods.length === 0) {
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
