import * as React from "react";
import * as ProgressPrimitive from "@radix-ui/react-progress";

import { cn } from "@/lib/utils";

function Progress({
  className,
  value,
  children,
  ...props
}: React.ComponentProps<typeof ProgressPrimitive.Root>) {
  const progressPercentage = value || 0;

  return (
    <ProgressPrimitive.Root
      data-slot="progress"
      className={cn(
        "bg-primary/20 relative h-2 w-full overflow-hidden rounded-full",
        className,
      )}
      {...props}
    >
      <ProgressPrimitive.Indicator
        data-slot="progress-indicator"
        className="bg-primary h-full w-full flex-1 transition-all"
        style={{
          transform: `translateX(-${100 - progressPercentage}%)`,
        }}
      />
      {children && (
        <>
          {/* Unfilled portion - text-primary */}
          <div
            className="text-primary absolute inset-0 z-10 flex items-center justify-center"
            aria-hidden="true"
          >
            {children}
          </div>
          {/* Filled portion - text-primary-foreground, clipped to progress width */}
          <div
            className="text-primary-foreground absolute inset-0 z-10 flex items-center justify-center transition-all"
            style={{
              clipPath: `inset(0 ${100 - progressPercentage}% 0 0)`,
            }}
          >
            {children}
          </div>
        </>
      )}
    </ProgressPrimitive.Root>
  );
}

export { Progress };
