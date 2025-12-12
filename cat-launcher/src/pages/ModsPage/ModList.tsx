import { Mod } from "@/generated-types/Mod";
import { GameVariant } from "@/generated-types/GameVariant";
import ModCard from "./ModCard";

interface ModListProps {
  mods: Mod[];
  variant: GameVariant;
}

export default function ModList({ mods, variant }: ModListProps) {
  if (mods.length === 0) {
    return (
      <div className="text-center py-8">
        <p className="text-muted-foreground">No mods available</p>
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {mods.map((modItem) => (
        <ModCard key={`${variant}-${modItem.id}`} mod={modItem} variant={variant} />
      ))}
    </div>
  );
}
