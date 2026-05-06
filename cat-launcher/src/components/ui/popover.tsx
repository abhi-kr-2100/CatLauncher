import * as React from "react";
import * as PopoverPrimitive from "@radix-ui/react-popover";

import { cn } from "@/lib/utils";

/**
 * The root component of a popover, which manages its open/closed state.
 * Built on top of Radix UI's Popover primitive.
 *
 * @param props - The properties for the popover root component.
 * @returns A React element representing the popover root.
 *
 * @public
 */
function Popover({
  ...props
}: React.ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root data-slot="popover" {...props} />;
}

/**
 * The trigger element that opens the popover when interacted with.
 *
 * @param props - The properties for the popover trigger component.
 * @returns A React element representing the popover trigger.
 *
 * @public
 */
function PopoverTrigger({
  ...props
}: React.ComponentProps<typeof PopoverPrimitive.Trigger>) {
  return (
    <PopoverPrimitive.Trigger
      data-slot="popover-trigger"
      {...props}
    />
  );
}

/**
 * The content that is displayed inside the popover when it is open.
 *
 * @param props - The properties for the popover content component, including alignment and side offset.
 * @returns A React element representing the popover content.
 *
 * @public
 */
function PopoverContent({
  className,
  align = "center",
  sideOffset = 4,
  ...props
}: React.ComponentProps<typeof PopoverPrimitive.Content>) {
  return (
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        data-slot="popover-content"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-72 origin-(--radix-popover-content-transform-origin) rounded-md border p-4 shadow-md outline-hidden",
          className,
        )}
        {...props}
      />
    </PopoverPrimitive.Portal>
  );
}

/**
 * An optional anchor element that the popover will position itself relative to.
 *
 * @param props - The properties for the popover anchor component.
 * @returns A React element representing the popover anchor.
 *
 * @public
 */
function PopoverAnchor({
  ...props
}: React.ComponentProps<typeof PopoverPrimitive.Anchor>) {
  return (
    <PopoverPrimitive.Anchor data-slot="popover-anchor" {...props} />
  );
}

export { Popover, PopoverTrigger, PopoverContent, PopoverAnchor };
