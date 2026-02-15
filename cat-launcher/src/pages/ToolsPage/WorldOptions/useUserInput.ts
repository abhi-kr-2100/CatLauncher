import { useState, useCallback } from "react";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useUserInput() {
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);
  const [selectedWorld, setSelectedWorld] = useState<string | null>(
    null,
  );

  const setVariant = useCallback((variant: GameVariant) => {
    setSelectedVariant(variant);
    setSelectedWorld(null);
  }, []);

  const setWorld = useCallback((world: string | null) => {
    setSelectedWorld(world);
  }, []);

  const reset = useCallback(() => {
    setSelectedVariant(null);
    setSelectedWorld(null);
  }, []);

  return {
    selectedVariant,
    selectedWorld,
    setVariant,
    setWorld,
    reset,
  };
}
