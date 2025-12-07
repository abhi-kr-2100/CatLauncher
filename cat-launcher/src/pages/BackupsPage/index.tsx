import { useState } from "react";

import { Combobox } from "@/components/ui/combobox";
import { useGameVariants } from "@/hooks/useGameVariants";
import { GameVariant } from "@/generated-types/GameVariant";
import { BackupsTable } from "./BackupsTable";

function BackupsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(
    null
  );

  return (
    <div className="flex flex-col gap-4 p-2">
      <Combobox
        items={gameVariants.map((v) => ({
          value: v.id,
          label: v.name,
        }))}
        value={selectedVariant ?? undefined}
        onChange={(value) => setSelectedVariant(value as GameVariant)}
        placeholder={
          gameVariantsLoading ? "Loading..." : "Select a game variant"
        }
        disabled={gameVariantsLoading}
        autoselect={true}
        className="w-72"
      />
      {selectedVariant && <BackupsTable variant={selectedVariant} />}
    </div>
  );
}

export default BackupsPage;
