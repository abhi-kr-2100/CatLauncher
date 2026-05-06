"use client";

import { useVirtualizer } from "@tanstack/react-virtual";
import { ChevronsUpDown } from "lucide-react";
import { ReactNode, useEffect, useRef, useState } from "react";

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

/**
 * Represents an individual item in the combobox.
 *
 * @public
 */
export interface ComboboxItem {
  /** The internal value of the item. */
  value: string;
  /** The display label of the item, which can be a React node. */
  label: ReactNode;
}

/**
 * Properties for the internal {@link VirtualizedCommand} component.
 *
 * @internal
 */
interface VirtualizedCommandProps {
  /** The height of the virtualized list. */
  height: string;
  /** The list of items to display. */
  items: ComboboxItem[];
  /** Placeholder text for the search input. */
  placeholder: string;
  /** The currently selected value. */
  value?: string;
  /** Callback function triggered when an item is selected. */
  onSelect?: (_value: string) => void;
}

/**
 * A virtualized command component for efficient rendering of large lists within a command palette.
 *
 * @param props - The properties for the virtualized command component.
 * @returns A React element representing the virtualized command.
 *
 * @internal
 */
const VirtualizedCommand = ({
  height,
  items,
  placeholder,
  value,
  onSelect,
}: VirtualizedCommandProps) => {
  const [filteredItems, setFilteredItems] =
    useState<ComboboxItem[]>(items);
  const [focusedIndex, setFocusedIndex] = useState(0);

  const parentRef = useRef<HTMLDivElement | null>(null);

  const virtualizer = useVirtualizer({
    count: filteredItems.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 35,
    overscan: 5,
  });

  const virtualOptions = virtualizer.getVirtualItems();

  const handleSearch = (search: string) => {
    setFilteredItems(
      items.filter((option) =>
        option.value.toLowerCase().includes(search.toLowerCase()),
      ),
    );
  };

  useEffect(() => {
    if (!value) {
      return;
    }

    const option = filteredItems.find(
      (option) => option.value === value,
    );
    if (!option) {
      return;
    }

    const index = filteredItems.indexOf(option);
    setFocusedIndex(index);
    virtualizer.scrollToIndex(index, {
      align: "center",
    });
  }, [value, filteredItems, virtualizer]);

  return (
    <Command shouldFilter={false}>
      <CommandInput
        onValueChange={handleSearch}
        placeholder={placeholder}
      />
      <CommandList
        ref={parentRef}
        style={{
          height: height,
          width: "100%",
          overflow: "auto",
        }}
      >
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup>
          <div
            style={{
              height: `${virtualizer.getTotalSize()}px`,
              width: "100%",
              position: "relative",
            }}
          >
            {virtualOptions.map((virtualOption) => (
              <CommandItem
                key={filteredItems[virtualOption.index].value}
                className={cn(
                  "absolute left-0 top-0 w-full bg-transparent",
                  focusedIndex === virtualOption.index &&
                    "bg-accent text-accent-foreground",
                )}
                style={{
                  height: `${virtualOption.size}px`,
                  transform: `translateY(${virtualOption.start}px)`,
                }}
                value={filteredItems[virtualOption.index].value}
                onMouseEnter={() =>
                  setFocusedIndex(virtualOption.index)
                }
                onMouseLeave={() => setFocusedIndex(-1)}
                onSelect={onSelect}
              >
                {filteredItems[virtualOption.index].label}
              </CommandItem>
            ))}
          </div>
        </CommandGroup>
      </CommandList>
    </Command>
  );
};

/**
 * Properties for the {@link VirtualizedCombobox} component.
 *
 * @public
 */
interface ComboboxProps {
  /** The list of items to display in the combobox. */
  items: ComboboxItem[];
  /** The currently selected value. */
  value?: string;
  /** An optional label to display above the combobox. */
  label?: string;
  /** Callback function triggered when the selection changes. */
  onChange: (value: string) => void;
  /** Placeholder text when no item is selected. */
  placeholder?: string;
  /** Whether the combobox is disabled. */
  disabled?: boolean;
  /**
   * Whether to automatically select an item.
   * Can be a boolean or a function that returns the item to select.
   */
  autoselect?:
    | boolean
    | ((items: ComboboxItem[]) => ComboboxItem | undefined);
  /** Additional CSS class names for the container. */
  className?: string;
}

/**
 * A virtualized combobox component for selecting an item from a potentially large list.
 * Combines a button trigger with a popover containing a virtualized list.
 *
 * @param props - The properties for the virtualized combobox.
 * @returns A React element representing the virtualized combobox.
 *
 * @public
 */
export function VirtualizedCombobox({
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
      <Popover open={open} onOpenChange={setOpen} modal={true}>
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
              <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </div>
          </Button>
        </PopoverTrigger>
        <PopoverContent className="p-0">
          <VirtualizedCommand
            height="400px"
            items={items}
            placeholder="Search..."
            value={value}
            onSelect={(currentValue) => {
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
          />
        </PopoverContent>
      </Popover>
    </div>
  );
}

export default VirtualizedCombobox;
