import { useQuery } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { listAllMods } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import ModCard from "./ModCard";
import { useEffect } from "react";
import { toastCL } from "@/lib/utils";

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

  useEffect(() => {
    if (error) {
      toastCL("error", "Failed to load mods.", error);
    }
  }, [error]);

  if (isLoading) {
    return <p className="text-muted-foreground">Loading mods...</p>;
  }

  if (!mods || mods.length === 0) {
    return <p className="text-muted-foreground">No mods available.</p>;
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {mods.map((mod) => (
        <ModCard
          key={`${variant}-${mod.type}-${mod.content.id}`}
          variant={variant}
          mod={mod}
        />
      ))}
    </div>
  );
}
