import {
  useQueryClient,
  UseMutationOptions,
  UseQueryOptions,
} from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import {
  fetchGameVariantsInfo,
  updateGameVariantOrder,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useApiMutation } from "./useApiMutation";
import { useApiQuery } from "./useApiQuery";

type GameVariantsQueryOptions<
  TQueryFnData = GameVariantInfo[],
  TError = Error,
  TData = GameVariantInfo[],
> = Omit<
  UseQueryOptions<TQueryFnData, TError, TData>,
  "queryKey" | "queryFn"
>;

type GameVariantsMutationOptions<
  TData = void,
  TError = Error,
  TVariables = {
    ids: GameVariant[];
    newItems: GameVariantInfo[];
  },
  TContext = unknown,
> = Omit<
  UseMutationOptions<TData, TError, TVariables, TContext>,
  "mutationFn"
>;

interface UseGameVariantsOptions<
  TQueryFnData = GameVariantInfo[],
  TQueryError = Error,
  TQueryData = GameVariantInfo[],
  TMutationData = void,
  TMutationError = Error,
  TMutationVariables = {
    ids: GameVariant[];
    newItems: GameVariantInfo[];
  },
  TMutationContext = { previousGameVariants?: GameVariantInfo[] },
> {
  queryOptions?: GameVariantsQueryOptions<
    TQueryFnData,
    TQueryError,
    TQueryData
  >;
  mutationOptions?: GameVariantsMutationOptions<
    TMutationData,
    TMutationError,
    TMutationVariables,
    TMutationContext
  >;
}

export function useGameVariants<
  TQueryFnData = GameVariantInfo[],
  TQueryError = Error,
  TQueryData = GameVariantInfo[],
  TMutationData = void,
  TMutationError = Error,
  TMutationVariables = {
    ids: GameVariant[];
    newItems: GameVariantInfo[];
  },
  TMutationContext = { previousGameVariants?: GameVariantInfo[] },
>({
  queryOptions,
  mutationOptions,
}: UseGameVariantsOptions<
  TQueryFnData,
  TQueryError,
  TQueryData,
  TMutationData,
  TMutationError,
  TMutationVariables,
  TMutationContext
> = {}) {
  const queryClient = useQueryClient();

  const {
    data: gameVariants = [],
    isLoading,
    isError,
    error,
  } = useApiQuery<TQueryFnData, TQueryError, TQueryData>({
    queryKey: queryKeys.gameVariantsInfo(),
    queryFn:
      fetchGameVariantsInfo as unknown as () => Promise<TQueryFnData>,
    ...queryOptions,
  });

  const { mutate } = useApiMutation<
    TMutationData,
    TMutationError,
    TMutationVariables,
    TMutationContext
  >({
    ...mutationOptions,
    mutationFn: ({ ids }: { ids: GameVariant[] }) =>
      updateGameVariantOrder(
        ids,
      ) as unknown as Promise<TMutationData>,
    onMutate: async (variables: TMutationVariables) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.gameVariantsInfo(),
      });

      const previousGameVariants = queryClient.getQueryData<
        GameVariantInfo[]
      >(queryKeys.gameVariantsInfo());

      queryClient.setQueryData<GameVariantInfo[]>(
        queryKeys.gameVariantsInfo(),
        (
          variables as {
            newItems: GameVariantInfo[];
          }
        ).newItems,
      );

      const context =
        (await mutationOptions?.onMutate?.(variables)) || {};

      return { previousGameVariants, ...context } as TMutationContext;
    },
    onError: (error, variables, context) => {
      if (
        (context as { previousGameVariants?: GameVariantInfo[] })
          ?.previousGameVariants
      ) {
        queryClient.setQueryData(
          queryKeys.gameVariantsInfo(),
          (context as { previousGameVariants: GameVariantInfo[] })
            .previousGameVariants,
        );
      }
      mutationOptions?.onError?.(error, variables, context);
    },
    onSettled: (data, error, variables, context) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.gameVariantsInfo(),
      });
      mutationOptions?.onSettled?.(data, error, variables, context);
    },
  });

  const updateOrder = (newOrder: GameVariantInfo[]) => {
    mutate({
      ids: newOrder.map((item) => item.id),
      newItems: newOrder,
    } as TMutationVariables);
  };

  return {
    gameVariants,
    updateOrder,
    isLoading,
    isError,
    error,
  };
}
