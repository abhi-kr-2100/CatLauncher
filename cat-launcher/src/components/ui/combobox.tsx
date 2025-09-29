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
import { cn } from "@/lib/utils";
import { CheckIcon, ChevronsUpDownIcon } from "lucide-react";
import { useEffect, useState } from "react";

export interface ComboboxItem {
  value: string;
  label: string;
}

interface ComboboxProps {
  items: ComboboxItem[];
  value?: string;
  label?: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  autoselect?: boolean;
}

export function Combobox({
  items,
  value,
  onChange,
  label,
  placeholder,
  disabled,
  autoselect,
}: ComboboxProps) {
  const [open, setOpen] = useState(false);

  // Auto-select the first item if autoselect is enabled.
  useEffect(() => {
    if (!autoselect || value || items.length === 0) {
      return;
    }

    const first = items[0].value;
    onChange(first);
  }, [autoselect, value, items, onChange]);

  return (
    <div>
      {label ? (
        <div className="text-sm text-muted-foreground mb-2">{label}</div>
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
            <div className="flex items-center gap-2">
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
                        (i) => i.value === currentValue
                      )?.value;
                      if (selectedValue === undefined) {
                        throw new Error(
                          "Combobox: Selected value not found in items"
                        );
                      }
                      onChange(selectedValue);
                      setOpen(false);
                    }}
                  >
                    <CheckIcon
                      className={cn({ hidden: value !== item.value })}
                    />
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
