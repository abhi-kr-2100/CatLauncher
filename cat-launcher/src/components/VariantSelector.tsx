import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { GameVariant } from "@/generated-types/GameVariant";

/**
 * Props for the {@link VariantSelector} component.
 */
interface VariantSelectorProps {
  /**
   * The list of available game variants.
   */
  gameVariants: GameVariantInfo[];
  /**
   * The currently selected game variant, or null if none is selected.
   */
  selectedVariant: GameVariant | null;
  /**
   * Callback fired when a game variant is selected.
   * @param _variant - The selected game variant.
   */
  onVariantChange: (_variant: GameVariant) => void;
  /**
   * Whether the game variants are currently being loaded.
   */
  isLoading: boolean;
  /**
   * Optional placeholder text to display in the selector.
   */
  placeholder?: string;
  /**
   * Whether the selector is disabled.
   * @defaultValue false
   */
  disabled?: boolean;
}

/**
 * A component that allows the user to select a game variant from a list.
 * Uses a {@link VirtualizedCombobox} for efficient rendering of large lists.
 *
 * @param props - The component props.
 * @returns A React component that renders the variant selector.
 */
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
        (isLoading ? "Loading..." : "Select a game variant")
      }
      disabled={disabled || isLoading}
      autoselect={true}
      className="w-2xs"
    />
  );
}
