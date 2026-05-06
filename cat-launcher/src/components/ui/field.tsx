import { useMemo, ComponentProps, ReactNode } from "react";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";

/**
 * A fieldset component for grouping related elements in a form.
 *
 * @param props - The properties for the fieldset component.
 * @returns A React element representing the fieldset.
 *
 * @public
 */
function FieldSet({
  className,
  ...props
}: ComponentProps<"fieldset">) {
  return (
    <fieldset
      data-slot="field-set"
      className={cn(
        "flex flex-col gap-6",
        "has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A legend component for a fieldset, providing a caption.
 *
 * @param props - The properties for the field legend component, including an optional visual `variant`.
 * @returns A React element representing the field legend.
 *
 * @public
 */
function FieldLegend({
  className,
  variant = "legend",
  ...props
}: ComponentProps<"legend"> & {
  variant?: "legend" | "label";
}) {
  return (
    <legend
      data-slot="field-legend"
      data-variant={variant}
      className={cn(
        "mb-3 font-medium",
        "data-[variant=legend]:text-base",
        "data-[variant=label]:text-sm",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A container for grouping fields together.
 *
 * @param props - The properties for the field group component.
 * @returns A React element representing the field group.
 *
 * @public
 */
function FieldGroup({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      data-slot="field-group"
      className={cn(
        "group/field-group @container/field-group flex w-full flex-col gap-7 data-[slot=checkbox-group]:gap-3 [&>[data-slot=field-group]]:gap-4",
        className,
      )}
      {...props}
    />
  );
}

/**
 * Defines the visual variants for the Field component, supporting different orientations.
 *
 * @public
 */
const fieldVariants = cva(
  "group/field flex w-full gap-3 data-[invalid=true]:text-destructive",
  {
    variants: {
      orientation: {
        vertical: ["flex-col [&>*]:w-full [&>.sr-only]:w-auto"],
        horizontal: [
          "flex-row items-center",
          "[&>[data-slot=field-label]]:flex-auto",
          "has-[>[data-slot=field-content]]:items-start has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
        ],
        responsive: [
          "flex-col [&>*]:w-full [&>.sr-only]:w-auto @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto",
          "@md/field-group:[&>[data-slot=field-label]]:flex-auto",
          "@md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
        ],
      },
    },
    defaultVariants: {
      orientation: "vertical",
    },
  },
);

/**
 * A wrapper for a single field, including label, input, description, and error message.
 *
 * @param props - The properties for the field component, including orientation.
 * @returns A React element representing the field.
 *
 * @public
 */
function Field({
  className,
  orientation = "vertical",
  ...props
}: ComponentProps<"div"> & VariantProps<typeof fieldVariants>) {
  return (
    <div
      role="group"
      data-slot="field"
      data-orientation={orientation}
      className={cn(fieldVariants({ orientation }), className)}
      {...props}
    />
  );
}

/**
 * A container for the main content of a field (e.g., the input).
 *
 * @param props - The properties for the field content component.
 * @returns A React element representing the field content.
 *
 * @public
 */
function FieldContent({
  className,
  ...props
}: ComponentProps<"div">) {
  return (
    <div
      data-slot="field-content"
      className={cn(
        "group/field-content flex flex-1 flex-col gap-1.5 leading-snug",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A label component specifically styled for use within a Field.
 *
 * @param props - The properties for the field label component.
 * @returns A React element representing the field label.
 *
 * @public
 */
function FieldLabel({
  className,
  ...props
}: ComponentProps<typeof Label>) {
  return (
    <Label
      data-slot="field-label"
      className={cn(
        "group/field-label peer/field-label flex w-fit gap-2 leading-snug group-data-[disabled=true]/field:opacity-50",
        "has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border [&>*]:data-[slot=field]:p-4",
        "has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A simpler title component for use within a Field when a full Label is not needed.
 *
 * @param props - The properties for the field title component.
 * @returns A React element representing the field title.
 *
 * @public
 */
function FieldTitle({ className, ...props }: ComponentProps<"div">) {
  return (
    <div
      data-slot="field-label"
      className={cn(
        "flex w-fit items-center gap-2 text-sm leading-snug font-medium group-data-[disabled=true]/field:opacity-50",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A description component for providing additional context for a field.
 *
 * @param props - The properties for the field description component.
 * @returns A React element representing the field description.
 *
 * @public
 */
function FieldDescription({
  className,
  ...props
}: ComponentProps<"p">) {
  return (
    <p
      data-slot="field-description"
      className={cn(
        "text-muted-foreground text-sm leading-normal font-normal group-has-[[data-orientation=horizontal]]/field:text-balance",
        "last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5",
        "[&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A separator component for visually dividing sections within a field group.
 *
 * @param props - The properties for the field separator component, which can optionally include children.
 * @returns A React element representing the field separator.
 *
 * @public
 */
function FieldSeparator({
  children,
  className,
  ...props
}: ComponentProps<"div"> & {
  children?: ReactNode;
}) {
  return (
    <div
      data-slot="field-separator"
      data-content={!!children}
      className={cn(
        "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2",
        className,
      )}
      {...props}
    >
      <Separator className="absolute inset-0 top-1/2" />
      {children && (
        <span
          className="bg-background text-muted-foreground relative mx-auto block w-fit px-2"
          data-slot="field-separator-content"
        >
          {children}
        </span>
      )}
    </div>
  );
}

/**
 * A component for displaying error messages related to a field.
 *
 * @param props - The properties for the field error component, including an optional `errors` array.
 * @returns A React element representing the field error, or null if no errors are present.
 *
 * @public
 */
function FieldError({
  className,
  children,
  errors,
  ...props
}: ComponentProps<"div"> & {
  errors?: Array<{ message?: string } | undefined>;
}) {
  const content = useMemo(() => {
    if (children) {
      return children;
    }

    if (!errors?.length) {
      return null;
    }

    const uniqueErrors = [
      ...new Map(
        errors.map((error) => [error?.message, error]),
      ).values(),
    ];

    if (uniqueErrors?.length == 1) {
      return uniqueErrors[0]?.message;
    }

    return (
      <ul className="ml-4 flex list-disc flex-col gap-1">
        {uniqueErrors.map(
          (error, index) =>
            error?.message && <li key={index}>{error.message}</li>,
        )}
      </ul>
    );
  }, [children, errors]);

  if (!content) {
    return null;
  }

  return (
    <div
      role="alert"
      data-slot="field-error"
      className={cn(
        "text-destructive text-sm font-normal",
        className,
      )}
      {...props}
    >
      {content}
    </div>
  );
}

export {
  Field,
  FieldLabel,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLegend,
  FieldSeparator,
  FieldSet,
  FieldContent,
  FieldTitle,
};
