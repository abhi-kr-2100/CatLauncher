import { ControllerRenderProps, FieldError } from "react-hook-form";

import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

import { WorldOptionsFormValues } from "../WorldOptionInput";

interface StringOptionProps {
  field: ControllerRenderProps<
    WorldOptionsFormValues,
    `options.${number}.value`
  >;
  error?: FieldError;
  placeholder?: string;
}

export function StringOption({
  field,
  error,
  placeholder,
}: StringOptionProps) {
  return (
    <Input
      {...field}
      className={cn(
        "max-w-xs",
        error && "border-destructive focus-visible:ring-destructive",
      )}
      placeholder={placeholder}
      type="text"
    />
  );
}
