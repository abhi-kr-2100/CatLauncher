import { ControllerRenderProps, FieldError } from "react-hook-form";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

import { WorldOptionsFormValues } from "../WorldOptionInput";

interface BooleanOptionProps {
  name: string;
  field: ControllerRenderProps<
    WorldOptionsFormValues,
    `options.${number}.value`
  >;
  error?: FieldError;
}

export function BooleanOption({
  name,
  field,
  error,
}: BooleanOptionProps) {
  return (
    <div className="flex items-center space-x-2">
      <Checkbox
        id={`option-${name}`}
        checked={field.value === "true"}
        onCheckedChange={(checked) =>
          field.onChange(checked ? "true" : "false")
        }
      />
      <Label
        htmlFor={`option-${name}`}
        className={cn(
          "text-xs text-muted-foreground",
          error && "text-destructive",
        )}
      >
        Enabled
      </Label>
    </div>
  );
}
