import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { GameVariant } from "@/generated-types/GameVariant";

interface VariantSelectorProps {
  gameVariants: GameVariantInfo[];
  selectedVariant: GameVariant | null;
  onVariantChange: (variant: GameVariant) => void;
  isLoading: boolean;
  placeholder?: string;
  disabled?: boolean;
}

export default function VariantSelector({
  gameVariants,
  selectedVariant,
  onVariantChange,
  isLoading,
  placeholder,
  disabled,
}: VariantSelectorProps) {
  const comboboxItems: ComboboxItem[] = gameVariants.map((v) => ({
    value: v.id,
    label: v.name,
  }));

  return (
    <VirtualizedCombobox
      items={comboboxItems}
      value={selectedVariant ?? undefined}
      onChange={(value) => onVariantChange(value as GameVariant)}
      placeholder={
        placeholder ?? (isLoading ? "Loading..." : "Select a game variant")
      }
      disabled={disabled || isLoading}
      autoselect={true}
      className="w-2xs"
    />
  );
}
