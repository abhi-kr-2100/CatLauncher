import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { ChevronsUpDownIcon } from "lucide-react";
import { ReactNode, useEffect, useState } from "react";

export interface ComboboxItem {
  value: string;
  label: ReactNode;
}

interface ComboboxProps {
  items: ComboboxItem[];
  value?: string;
  label?: string;
  onChange: (_value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  autoselect?:
    | boolean
    | ((_items: ComboboxItem[]) => ComboboxItem | undefined);
  className?: string;
}

export function Combobox({
  items,
  value,
  onChange,
  label,
  placeholder,
  disabled,
  autoselect,
  className,
}: ComboboxProps) {
  const [open, setOpen] = useState(false);

  // Auto-select an item if autoselect is enabled.
  useEffect(() => {
    if (value || items.length === 0 || !autoselect) {
      return;
    }

    let selectedValue: string | undefined;

    if (typeof autoselect === "function") {
      selectedValue = autoselect(items)?.value;
    } else if (autoselect) {
      selectedValue = items[0]?.value;
    }

    if (selectedValue) {
      onChange(selectedValue);
    }
  }, [autoselect, value, items, onChange]);

  return (
    <div className={className}>
      {label ? (
        <div className="text-sm text-muted-foreground mb-2">
          {label}
        </div>
      ) : null}
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="outline"
            role="combobox"
            aria-expanded={open}
            disabled={disabled}
            className="w-full"
          >
            <div className="flex items-center justify-between w-full">
              {value
                ? items.find((i) => i.value === value)?.label
                : placeholder}
              <ChevronsUpDownIcon />
            </div>
          </Button>
        </PopoverTrigger>
        <PopoverContent>
          <Command>
            <CommandInput placeholder="Search..." />
            <CommandList>
              <CommandEmpty>No results found.</CommandEmpty>
              <CommandGroup>
                {items.map((item) => (
                  <CommandItem
                    key={item.value}
                    value={item.value}
                    onSelect={(currentValue: string) => {
                      const selectedValue = items.find(
                        (i) => i.value === currentValue,
                      )?.value;
                      if (selectedValue === undefined) {
                        throw new Error(
                          "Combobox: Selected value not found in items",
                        );
                      }
                      onChange(selectedValue);
                      setOpen(false);
                    }}
                  >
                    {item.label}
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>
    </div>
  );
}

export default Combobox;
