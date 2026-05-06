import { useMemo, useState } from "react";

import { useDebounce } from "./useDebounce";

/**
 * Options for the {@link useSearch} hook.
 */
export interface UseSearchOptions<T> {
  /**
   * The delay in milliseconds for debouncing the search query. Defaults to 300.
   */
  debounceDelay?: number;
  /**
   * A function that determines if an item matches the search query.
   *
   * @param item - The item to test.
   * @param query - The normalized (trimmed and lowercased) search query.
   * @returns True if the item matches the query, false otherwise.
   */
  searchFn?: (item: T, query: string) => boolean;
}

/**
 * A custom hook that provides searching and filtering functionality for a list of items.
 * It includes debouncing and normalization of the search query.
 *
 * @param items - The list of items to search through.
 * @param options - Optional search configuration.
 * @returns An object containing the search query, a setter for the query, filtered items, and search status.
 */
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
