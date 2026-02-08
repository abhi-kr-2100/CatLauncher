import { ControllerRenderProps, FieldError } from "react-hook-form";

import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { cn } from "@/lib/utils";

import { WorldOptionsFormValues } from "../WorldOptionInput";

interface EnumOptionProps {
  options: Array<{ value: string; label: string }>;
  field: ControllerRenderProps<
    WorldOptionsFormValues,
    `options.${number}.value`
  >;
  error?: FieldError;
}

export function EnumOption({
  options,
  field,
  error,
}: EnumOptionProps) {
  return (
    <VirtualizedCombobox
      items={options}
      value={field.value}
      onChange={field.onChange}
      placeholder="Select value..."
      className={cn("w-full max-w-xs", error && "border-destructive")}
    />
  );
}
