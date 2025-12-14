import { useMemo, useState } from "react";

import { useDebounce } from "./useDebounce";

export interface UseSearchOptions<T> {
  debounceDelay?: number;
  searchFn?: (item: T, query: string) => boolean;
}

export function useSearch<T>(
  items: T[],
  options: UseSearchOptions<T> = {},
) {
  const { debounceDelay = 300, searchFn } = options;

  const [searchInput, setSearchInput] = useState("");

  const debouncedSearchQuery = useDebounce(
    searchInput,
    debounceDelay,
  );
  const normalizedSearchQuery = useMemo(() => {
    return debouncedSearchQuery.trim().toLowerCase();
  }, [debouncedSearchQuery]);

  const filteredItems = useMemo(() => {
    if (!normalizedSearchQuery || !searchFn) {
      return items;
    }

    return items.filter((item) =>
      searchFn(item, normalizedSearchQuery),
    );
  }, [items, normalizedSearchQuery, searchFn]);

  return {
    searchQuery: searchInput,
    setSearchQuery: setSearchInput,
    filteredItems,
    hasActiveSearch: normalizedSearchQuery.length > 0,
  };
}
