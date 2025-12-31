import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
  QueryKey,
} from "@tanstack/react-query";

export function useApiQuery<
  TQueryFnData = unknown,
  TError = Error,
  TData = TQueryFnData,
  TQueryKey extends QueryKey = QueryKey,
>(
  options: UseQueryOptions<TQueryFnData, TError, TData, TQueryKey>,
): UseQueryResult<TData, TError> {
  return useQuery(options);
}
