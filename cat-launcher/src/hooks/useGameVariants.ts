import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import {
  fetchGameVariantsInfo,
  updateGameVariantOrder,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

interface UseGameVariantsOptions {
  onOrderUpdateError?: (error: unknown) => void;
  onFetchError?: (error: unknown) => void;
}

export function useGameVariants({
  onOrderUpdateError,
  onFetchError,
}: UseGameVariantsOptions = {}) {
  const queryClient = useQueryClient();
  const onFetchErrorRef = useRef(onFetchError);
  const onOrderUpdateErrorRef = useRef(onOrderUpdateError);

  useEffect(() => {
    onFetchErrorRef.current = onFetchError;
  }, [onFetchError]);

  useEffect(() => {
    onOrderUpdateErrorRef.current = onOrderUpdateError;
  }, [onOrderUpdateError]);

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
      onFetchErrorRef.current?.(error);
    }
  }, [isError, error]);

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
      onOrderUpdateErrorRef.current?.(error);
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
