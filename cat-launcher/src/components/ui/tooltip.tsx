"use client";

import * as React from "react";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";

import { cn } from "@/lib/utils";

/**
 * Provider component for the tooltip system. Wrap your app or section with this.
 * @public
 */
const TooltipProvider = TooltipPrimitive.Provider;

/**
 * Root component for an individual tooltip.
 * @public
 */
const Tooltip = TooltipPrimitive.Root;

/**
 * Element that triggers the tooltip on hover, focus, or tap.
 * @public
 */
const TooltipTrigger = TooltipPrimitive.Trigger;

/**
 * The content displayed within the tooltip when it is active.
 *
 * @param props - Tooltip content props.
 * @returns A React element representing the tooltip content.
 * @public
 */
const TooltipContent = React.forwardRef<
  React.ElementRef<typeof TooltipPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Content>
>(({ className, sideOffset = 4, ...props }, ref) => (
  <TooltipPrimitive.Content
    ref={ref}
    sideOffset={sideOffset}
    className={cn(
      "z-50 overflow-hidden rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
      className,
    )}
    {...props}
  />
));
TooltipContent.displayName = TooltipPrimitive.Content.displayName;

export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider };
