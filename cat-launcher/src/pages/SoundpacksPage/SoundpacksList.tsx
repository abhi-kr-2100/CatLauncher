import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import SoundpackCard from "./SoundpackCard";
import { useListAllSoundpacks } from "./hooks";

interface SoundpacksListProps {
  variant: GameVariant;
}

export default function SoundpacksList({
  variant,
}: SoundpacksListProps) {
  const { soundpacks, isLoading, error } =
    useListAllSoundpacks(variant);

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

  if (!soundpacks || soundpacks.length === 0) {
    return (
      <p className="text-muted-foreground">
        No soundpacks available.
      </p>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {soundpacks.map((soundpack) => (
        <SoundpackCard
          key={`${variant}-${soundpack.content.id}`}
          variant={variant}
          soundpack={soundpack}
        />
      ))}
    </div>
  );
}
