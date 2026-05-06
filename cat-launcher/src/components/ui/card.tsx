import * as React from "react";

import { cn } from "@/lib/utils";

/**
 * A container component for grouping related content and actions.
 *
 * @param props - Div element props.
 * @returns A React element representing the card.
 * @public
 */
function Card({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card"
      className={cn(
        "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
        className,
      )}
      {...props}
    />
  );
}

/**
 * Header section of a {@link Card}, typically containing {@link CardTitle} and {@link CardDescription}.
 *
 * @param props - Div element props.
 * @returns A React element representing the card header.
 * @public
 */
function CardHeader({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-header"
      className={cn(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
        className,
      )}
      {...props}
    />
  );
}

/**
 * The title component for a {@link Card}.
 *
 * @param props - Div element props.
 * @returns A React element representing the card title.
 * @public
 */
function CardTitle({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-title"
      className={cn("leading-none font-semibold", className)}
      {...props}
    />
  );
}

/**
 * The description component for a {@link Card}, providing additional details.
 *
 * @param props - Div element props.
 * @returns A React element representing the card description.
 * @public
 */
function CardDescription({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-description"
      className={cn("text-muted-foreground text-sm", className)}
      {...props}
    />
  );
}

/**
 * An optional action area within the {@link CardHeader}, positioned to the right of the title.
 *
 * @param props - Div element props.
 * @returns A React element representing the card action.
 * @public
 */
function CardAction({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-action"
      className={cn(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        className,
      )}
      {...props}
    />
  );
}

/**
 * The main content area of a {@link Card}.
 *
 * @param props - Div element props.
 * @returns A React element representing the card content.
 * @public
 */
function CardContent({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-content"
      className={cn("px-6", className)}
      {...props}
    />
  );
}

/**
 * The footer section of a {@link Card}, typically used for secondary actions.
 *
 * @param props - Div element props.
 * @returns A React element representing the card footer.
 * @public
 */
function CardFooter({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-footer"
      className={cn(
        "flex items-center px-6 [.border-t]:pt-6",
        className,
      )}
      {...props}
    />
  );
}

export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
};
