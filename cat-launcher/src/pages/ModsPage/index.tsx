import { useMemo, useState } from "react";

import { Combobox } from "@/components/ui/combobox";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useThirdPartyMods } from "@/hooks/useThirdPartyMods";
import { toastCL } from "@/lib/utils";
import ThirdPartyModCard from "./ThirdPartyModCard";

function ModsPage() {
  const {
    gameVariants,
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useGameVariants();
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(
    null,
  );

  const activeVariant = useMemo(() => {
    if (selectedVariant) {
      return selectedVariant;
    }

    return gameVariants[0]?.id ?? null;
  }, [selectedVariant, gameVariants]);

  const {
    mods,
    isLoading: modsLoading,
    isError: modsError,
    error: modsErrorObj,
    markInstalled,
    removeInstallation,
    installingModId,
    uninstallingModId,
  } = useThirdPartyMods(activeVariant, {
    onInstallSuccess: () => toastCL("success", "Mod marked as installed"),
    onInstallError: (error) =>
      toastCL("error", "Failed to mark mod as installed", error),
    onUninstallSuccess: () => toastCL("success", "Marked mod as uninstalled"),
    onUninstallError: (error) =>
      toastCL("error", "Failed to mark mod as uninstalled", error),
  });

  const variantItems = gameVariants.map((variant) => ({
    value: variant.id,
    label: variant.name,
  }));

  const modsErrorMessage = (() => {
    if (!modsError || !modsErrorObj) {
      return null;
    }

    if (modsErrorObj instanceof Error) {
      return modsErrorObj.message;
    }

    return "Unknown error";
  })();

  if (gameVariantsError) {
    return (
      <div className="p-4">
        Error loading game variants:{" "}
        {gameVariantsErrorObj instanceof Error
          ? gameVariantsErrorObj.message
          : "Unknown error"}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4 p-2">
      <div className="flex flex-wrap items-center gap-4">
        <Combobox
          items={variantItems}
          value={selectedVariant ?? undefined}
          onChange={(value) => setSelectedVariant(value as GameVariant)}
          placeholder={
            gameVariantsLoading ? "Loading variants..." : "Select a variant"
          }
          disabled={gameVariantsLoading || !gameVariants.length}
          autoselect={true}
          className="w-72"
        />
      </div>

      {!gameVariants.length && !gameVariantsLoading ? (
        <p className="text-sm text-muted-foreground">
          No game variants available. Configure a variant to view mods.
        </p>
      ) : modsLoading ? (
        <p>Loading mods...</p>
      ) : !activeVariant ? (
        <p className="text-sm text-muted-foreground">
          Select a game variant to view available mods.
        </p>
      ) : modsErrorMessage ? (
        <p className="text-sm text-destructive">{modsErrorMessage}</p>
      ) : mods.length === 0 ? (
        <p className="text-sm text-muted-foreground">
          No curated third-party mods found for this variant yet.
        </p>
      ) : (
        <div className="grid grid-cols-1 gap-2 md:grid-cols-2 xl:grid-cols-3">
          {mods.map((mod) => (
            <ThirdPartyModCard
              key={mod.id}
              mod={mod}
              onInstall={() => markInstalled(mod.id)}
              onUninstall={() => removeInstallation(mod.id)}
              isInstalling={installingModId === mod.id}
              isUninstalling={uninstallingModId === mod.id}
            />
          ))}
        </div>
      )}
    </div>
  );
}

export default ModsPage;
