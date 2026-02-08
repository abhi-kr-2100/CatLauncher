import { useMemo } from "react";
import {
  Controller,
  Control,
  RegisterOptions,
} from "react-hook-form";

import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from "@/components/ui/field";
import { WorldOption } from "@/generated-types/WorldOption";
import { WorldOptionMetadata } from "@/generated-types/WorldOptionMetadata";
import { cn } from "@/lib/utils";

import { BooleanOption } from "./inputs/BooleanOption";
import { EnumOption } from "./inputs/EnumOption";
import { NumberOption } from "./inputs/NumberOption";
import { StringOption } from "./inputs/StringOption";

export interface WorldOptionsFormValues {
  options: WorldOption[];
}

interface WorldOptionInputProps {
  option: WorldOption;
  index: number;
  control: Control<WorldOptionsFormValues>;
  metadata: WorldOptionMetadata;
}

export function WorldOptionInput({
  option,
  index,
  control,
  metadata,
}: WorldOptionInputProps) {
  const validationRules = useMemo(() => {
    const rules: RegisterOptions<
      WorldOptionsFormValues,
      `options.${number}.value`
    > = {};

    if (metadata.type === "number") {
      rules.validate = (v: string) => {
        const trimmed = v.trim();
        if (trimmed === "") return "Must be a valid number";

        const num = Number(trimmed);
        if (isNaN(num)) return "Must be a valid number";

        if (metadata.min !== null && num < metadata.min)
          return `Minimum value is ${metadata.min}`;

        if (metadata.max !== null && num > metadata.max)
          return `Maximum value is ${metadata.max}`;

        return true;
      };
    }

    return rules;
  }, [metadata.type, metadata.min, metadata.max]);

  return (
    <Controller
      name={`options.${index}.value`}
      control={control}
      rules={validationRules}
      render={({ field, fieldState: { error, isDirty } }) => (
        <Field className="pb-4">
          <FieldLabel
            className={cn(
              "text-sm font-semibold",
              error && "text-destructive",
              isDirty && "text-primary",
            )}
          >
            {metadata.name}
            {error && (
              <span className="ml-2 font-normal text-destructive">
                (Invalid)
              </span>
            )}
            {isDirty && !error && (
              <span className="ml-2 font-normal text-primary">
                (Modified)
              </span>
            )}
          </FieldLabel>
          <FieldDescription className="text-xs">
            {metadata.description}
          </FieldDescription>
          <FieldContent className="mt-2">
            {metadata.type === "boolean" ? (
              <BooleanOption
                name={option.name}
                field={field}
                error={error}
              />
            ) : metadata.type === "enum" ? (
              <EnumOption
                options={metadata.options ?? []}
                field={field}
                error={error}
              />
            ) : metadata.type === "number" ? (
              <NumberOption
                field={field}
                error={error}
                placeholder={option.default}
                min={metadata.min}
                max={metadata.max}
              />
            ) : (
              <StringOption
                field={field}
                error={error}
                placeholder={option.default}
              />
            )}
            {error && <FieldError errors={[error]} />}
            <div className="mt-1 text-[10px] text-muted-foreground italic">
              {option.default}
            </div>
          </FieldContent>
        </Field>
      )}
    />
  );
}
