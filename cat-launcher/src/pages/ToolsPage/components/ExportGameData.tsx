import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { join } from "@tauri-apps/api/path";
import { Download } from "lucide-react";

import { useGameVariants } from "@/hooks/useGameVariants";
import VariantSelector from "@/components/VariantSelector";
import { Button } from "@/components/ui/button";
import { exportGameData } from "@/lib/commands";
import { toastCL } from "@/lib/utils";
import { GameVariant } from "@/generated-types/GameVariant";

export default function ExportGameData() {
  const { gameVariants, isLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  const handleExport = async () => {
    if (!selectedVariant) {
      toastCL("warning", "Please select a game variant first");
      return;
    }

    try {
      const destinationDir = await open({
        directory: true,
        multiple: false,
        title: "Select Destination Directory",
      });

      if (destinationDir) {
        const timestamp = Math.floor(Date.now() / 1000);
        const fileName = `${selectedVariant}_data_${timestamp}.zip`;
        const destinationPath = await join(destinationDir, fileName);

        toastCL("info", "Exporting game data...");
        await exportGameData(selectedVariant, destinationPath);
        toastCL(
          "success",
          `Game data exported to ${destinationPath}`,
        );
      }
    } catch (error) {
      toastCL("error", "Failed to export game data", error);
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h2 className="text-2xl font-bold">Export Game Data</h2>
        <p className="text-muted-foreground">
          Zip and save your game data (saves, configs, etc.) to a
          location of your choice.
        </p>
      </div>

      <div className="space-y-4 max-w-md">
        <div className="space-y-2">
          <label className="text-sm font-medium">
            Select Game Variant
          </label>
          <VariantSelector
            gameVariants={gameVariants}
            selectedVariant={selectedVariant}
            onVariantChange={setSelectedVariant}
            isLoading={isLoading}
          />
        </div>

        <div className="pt-4">
          <Button
            onClick={handleExport}
            disabled={!selectedVariant}
            className="w-full"
          >
            <Download className="mr-2 h-4 w-4" />
            Select Location & Export
          </Button>
        </div>
      </div>
    </div>
  );
}
