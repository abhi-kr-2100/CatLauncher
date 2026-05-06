import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import {
  fetchGameVariantsInfo,
  updateGameVariantOrder,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useEffect } from "react";

/**
 * Options for the {@link useGameVariants} hook.
 */
interface UseGameVariantsOptions {
  /**
   * Callback function executed when updating the variant order fails.
   */
  onOrderUpdateError?: (error: unknown) => void;
  /**
   * Callback function executed when fetching variants fails.
   */
  onFetchError?: (error: unknown) => void;
}

/**
 * A custom hook that manages the list of game variants and their display order.
 *
 * @param options - Optional callbacks for order update and fetch errors.
 * @returns An object containing the list of game variants, an update function, and state information.
 */
export function useGameVariants({
  onOrderUpdateError,
  onFetchError,
}: UseGameVariantsOptions = {}) {
  const queryClient = useQueryClient();

  const {
    data: gameVariants = [],
    isLoading,
    isError,
    error,
  } = useQuery<GameVariantInfo[]>({
    queryKey: queryKeys.gameVariantsInfo(),
    queryFn: fetchGameVariantsInfo,
  });

  useEffect(() => {
    if (isError) {
      onFetchError?.(error);
    }
  }, [isError, error, onFetchError]);

  const { mutate } = useMutation({
    mutationFn: ({
      ids,
    }: {
      ids: GameVariant[];
      newItems: GameVariantInfo[];
    }) => updateGameVariantOrder(ids),
    onMutate: async ({ newItems }) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.gameVariantsInfo(),
      });

      const previousGameVariants = queryClient.getQueryData<
        GameVariantInfo[]
      >(queryKeys.gameVariantsInfo());

      queryClient.setQueryData<GameVariantInfo[]>(
        queryKeys.gameVariantsInfo(),
        newItems,
      );

      return { previousGameVariants };
    },
    onError: (error, _variables, context) => {
      if (context?.previousGameVariants) {
        queryClient.setQueryData(
          queryKeys.gameVariantsInfo(),
          context.previousGameVariants,
        );
      }
      onOrderUpdateError?.(error);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.gameVariantsInfo(),
      });
    },
  });

  const updateOrder = (newOrder: GameVariantInfo[]) => {
    mutate({
      ids: newOrder.map((item) => item.id),
      newItems: newOrder,
    });
  };

  return {
    gameVariants,
    updateOrder,
    isLoading,
    isError,
    error,
  };
}
