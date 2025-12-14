import { useQuery } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getModsActivity, listAllMods } from "@/lib/commands";
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
    isLoading: isLoadingMods,
    error: modsError,
  } = useQuery({
    queryKey: queryKeys.mods.listAll(variant),
    queryFn: () => listAllMods(variant),
  });

  const modIds = mods?.map((mod) => mod.content.id) ?? [];

  const {
    data: activities,
    isLoading: isLoadingActivities,
    error: activitiesError,
  } = useQuery({
    queryKey: ["modsActivity", variant, modIds],
    queryFn: () => getModsActivity(modIds, variant),
    enabled: modIds.length > 0,
  });

  useEffect(() => {
    if (modsError) {
      toastCL("error", "Failed to load mods.", modsError);
    }
    if (activitiesError) {
      toastCL(
        "error",
        "Failed to load mod activities.",
        activitiesError,
      );
    }
  }, [modsError, activitiesError]);

  const isLoading = isLoadingMods || isLoadingActivities;

  if (isLoading) {
    return <p className="text-muted-foreground">Loading mods...</p>;
  }

  if (!mods || mods.length === 0) {
    return (
      <p className="text-muted-foreground">No mods available.</p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {mods.map((mod) => (
        <ModCard
          key={`${variant}-${mod.type}-${mod.content.id}`}
          variant={variant}
          mod={mod}
          activity={activities?.[mod.content.id]}
        />
      ))}
    </div>
  );
}
