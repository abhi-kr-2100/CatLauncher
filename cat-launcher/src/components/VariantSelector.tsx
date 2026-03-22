import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { GameVariant } from "@/generated-types/GameVariant";
import { UI_STRINGS } from "@/lib/constants";

interface VariantSelectorProps {
  gameVariants: GameVariantInfo[];
  selectedVariant: GameVariant | null;
  onVariantChange: (_variant: GameVariant) => void;
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
        placeholder ??
        (isLoading
          ? UI_STRINGS.VARIANT_SELECTOR.LOADING
          : UI_STRINGS.VARIANT_SELECTOR.PLACEHOLDER)
      }
      disabled={disabled || isLoading}
      autoselect={true}
      className="w-2xs"
    />
  );
}
