import { ChevronDown } from "lucide-react";
import * as React from "react";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

interface DropdownButtonProps extends React.ComponentProps<
  typeof Button
> {
  options: {
    id: string;

    label: string;

    onClick: () => void;

    disabled?: boolean;

    tooltip?: string;
  }[];

  mainButtonDisabled?: boolean;
}

export function DropdownButton({
  children,

  onClick,

  disabled,

  mainButtonDisabled,

  options,

  className,

  variant = "default",

  size = "default",

  ...props
}: DropdownButtonProps) {
  return (
    <div
      className={cn(
        "inline-flex items-center rounded-md border shadow-sm",

        variant === "secondary" && "border-none shadow-none",

        variant === "ghost" && "border-none shadow-none",

        variant === "link" && "border-none shadow-none",

        className,
      )}
    >
      <Button
        {...props}
        variant={variant}
        size={size}
        disabled={disabled || mainButtonDisabled}
        onClick={onClick}
        className={cn(
          "grow rounded-r-none border-0 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0",

          "focus-visible:z-10",
        )}
      >
        {children}
      </Button>

      <div
        className={cn(
          "h-3/5 w-[1px]",

          variant === "default"
            ? "bg-primary-foreground/30"
            : "bg-border",

          (variant === "ghost" || variant === "link") &&
            "bg-transparent",
        )}
      />

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            {...props}
            variant={variant}
            size={size}
            disabled={disabled || options.length === 0}
            className={cn(
              "shrink-0 rounded-l-none border-0 px-2 shadow-none focus-visible:ring-0 focus-visible:ring-offset-0",

              "focus-visible:z-10",
            )}
          >
            <ChevronDown className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent align="end">
          {options.map((option) => {
            const item = (
              <DropdownMenuItem
                key={option.id}
                onClick={option.onClick}
                disabled={option.disabled}
              >
                {option.label}
              </DropdownMenuItem>
            );

            if (option.tooltip) {
              return (
                <Tooltip key={option.id}>
                  <TooltipTrigger asChild>
                    <span>{item}</span>
                  </TooltipTrigger>

                  <TooltipContent side="left">
                    <p>{option.tooltip}</p>
                  </TooltipContent>
                </Tooltip>
              );
            }

            return item;
          })}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
