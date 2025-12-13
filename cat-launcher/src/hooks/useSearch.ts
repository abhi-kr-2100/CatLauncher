import { useMemo, useState } from "react";

import { useDebounce } from "./useDebounce";

export interface UseSearchOptions<T> {
  debounceDelay?: number;
  searchFn?: (item: T, query: string) => boolean;
  searchInput?: string;
  setSearchInput?: (value: string) => void;
}

export function useSearch<T>(
  items: T[],
  options: UseSearchOptions<T> = {},
) {
  const {
    debounceDelay = 300,
    searchFn,
    searchInput: externalSearchInput,
    setSearchInput: externalSetSearchInput,
  } = options;

  const [internalSearchInput, setInternalSearchInput] = useState("");

  const searchInput =
    externalSearchInput !== undefined
      ? externalSearchInput
      : internalSearchInput;
  const setSearchInput =
    externalSetSearchInput || setInternalSearchInput;

  const debouncedSearchQuery = useDebounce(
    searchInput,
    debounceDelay,
  );

  const filteredItems = useMemo(() => {
    if (!debouncedSearchQuery.trim() || !searchFn) {
      return items;
    }

    return items.filter((item) =>
      searchFn(item, debouncedSearchQuery),
    );
  }, [items, debouncedSearchQuery, searchFn]);

  return {
    searchInput,
    setSearchInput,
    debouncedSearchQuery,
    filteredItems,
    hasActiveSearch: debouncedSearchQuery.trim().length > 0,
  };
}
