import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload } from "lucide-react";

import { useGameVariants } from "@/hooks/useGameVariants";
import VariantSelector from "@/components/VariantSelector";
import { Button } from "@/components/ui/button";
import { importGameData } from "@/lib/commands";
import { toastCL } from "@/lib/utils";
import { GameVariant } from "@/generated-types/GameVariant";

export default function ImportGameData() {
  const { gameVariants, isLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  const handleImport = async (type: "zip" | "folder") => {
    if (!selectedVariant) {
      toastCL("warning", "Please select a game variant first");
      return;
    }

    try {
      const sourcePath = await open({
        directory: type === "folder",
        multiple: false,
        filters:
          type === "zip"
            ? [{ name: "Zip Archive", extensions: ["zip"] }]
            : [],
        title: `Select ${type === "zip" ? "Zip Archive" : "Game Folder"}`,
      });

      if (sourcePath) {
        toastCL(
          "info",
          "Importing game data... This might take a while.",
        );
        await importGameData(selectedVariant, sourcePath);
        toastCL("success", "Game data imported successfully");
      }
    } catch (error) {
      toastCL("error", "Failed to import game data", error);
    }
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h2 className="text-2xl font-bold">Import Game Data</h2>
        <p className="text-muted-foreground">
          Import game data from a zip archive or a folder. A backup
          will be created before importing.
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

        <div className="pt-4 space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <Button
              variant="outline"
              onClick={() => handleImport("zip")}
              disabled={!selectedVariant}
            >
              <Upload className="mr-2 h-4 w-4" />
              From Zip
            </Button>
            <Button
              variant="outline"
              onClick={() => handleImport("folder")}
              disabled={!selectedVariant}
            >
              <Upload className="mr-2 h-4 w-4" />
              From Folder
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
