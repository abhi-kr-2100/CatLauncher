import { Combobox } from "@/components/ui/combobox";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { GameVariant } from "@/generated-types/GameVariant";
import { cn } from "@/lib/utils";

interface VariantSelectorProps {
  gameVariants: GameVariantInfo[];
  selectedVariant: GameVariant | null;
  onVariantChange: (variant: GameVariant) => void;
  isLoading: boolean;
  className?: string;
  placeholder?: string;
  disabled?: boolean;
}

export default function VariantSelector({
  gameVariants,
  selectedVariant,
  onVariantChange,
  isLoading,
  className,
  placeholder,
  disabled,
}: VariantSelectorProps) {
  return (
    <Combobox
      items={gameVariants.map((v) => ({
        value: v.id,
        label: v.name,
      }))}
      value={selectedVariant ?? undefined}
      onChange={(value) => onVariantChange(value as GameVariant)}
      placeholder={
        placeholder ?? (isLoading ? "Loading..." : "Select a game variant")
      }
      disabled={disabled ?? isLoading}
      autoselect={true}
      className={cn("w-72", className)}
    />
  );
}
