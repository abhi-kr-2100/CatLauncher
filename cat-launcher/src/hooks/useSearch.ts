import { useCallback, useMemo, useState } from "react";

import { useDebounce } from "./useDebounce";

export interface SearchableItem {
  id: string;
  name: string;
  description: string;
}

export interface UseSearchOptions<T extends SearchableItem> {
  debounceDelay?: number;
  searchFn?: (item: T, query: string) => boolean;
}

export function useSearch<T extends SearchableItem>(
  items: T[],
  options: UseSearchOptions<T> = {},
) {
  const { debounceDelay = 300, searchFn } = options;

  const [searchInput, setSearchInput] = useState("");
  const debouncedSearchQuery = useDebounce(searchInput, debounceDelay);

  const defaultSearchFn = useCallback(
    (item: SearchableItem, query: string): boolean => {
      const lowerQuery = query.toLowerCase().trim();
      return (
        item.name.toLowerCase().includes(lowerQuery) ||
        item.description.toLowerCase().includes(lowerQuery) ||
        item.id.toLowerCase().includes(lowerQuery)
      );
    },
    [],
  );

  const filteredItems = useMemo(() => {
    if (!debouncedSearchQuery.trim()) {
      return items;
    }

    const searchFunction = searchFn || defaultSearchFn;
    return items.filter((item) => searchFunction(item, debouncedSearchQuery));
  }, [items, debouncedSearchQuery, searchFn, defaultSearchFn]);

  return {
    searchInput,
    setSearchInput,
    debouncedSearchQuery,
    filteredItems,
    hasActiveSearch: debouncedSearchQuery.trim().length > 0,
  };
}
