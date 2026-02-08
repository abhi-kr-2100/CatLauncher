import { ControllerRenderProps, FieldError } from "react-hook-form";

import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

import { WorldOptionsFormValues } from "../WorldOptionInput";

interface NumberOptionProps {
  field: ControllerRenderProps<
    WorldOptionsFormValues,
    `options.${number}.value`
  >;
  error?: FieldError;
  placeholder?: string;
  min?: number | null;
  max?: number | null;
}

export function NumberOption({
  field,
  error,
  placeholder,
  min,
  max,
}: NumberOptionProps) {
  return (
    <Input
      {...field}
      className={cn(
        "max-w-xs",
        error && "border-destructive focus-visible:ring-destructive",
      )}
      placeholder={placeholder}
      type="number"
      inputMode="decimal"
      step="any"
      min={min ?? undefined}
      max={max ?? undefined}
    />
  );
}
