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
  onOrderUpdateError?: (error: Error) => void;
  onFetchError?: (error: Error) => void;
}

export function useGameVariants({
  onOrderUpdateError,
  onFetchError,
}: UseGameVariantsOptions = {}) {
  const queryClient = useQueryClient();

  const onFetchErrorRef = useRef(onFetchError);
  useEffect(() => {
    onFetchErrorRef.current = onFetchError;
  }, [onFetchError]);

  const onOrderUpdateErrorRef = useRef(onOrderUpdateError);
  useEffect(() => {
    onOrderUpdateErrorRef.current = onOrderUpdateError;
  }, [onOrderUpdateError]);

  const {
    data: gameVariants = [],
    isLoading,
    error,
  } = useQuery<GameVariantInfo[], Error>({
    queryKey: queryKeys.gameVariantsInfo(),
    queryFn: fetchGameVariantsInfo,
  });

  useEffect(() => {
    if (error) {
      onFetchErrorRef.current?.(error);
    }
  }, [error]);

  const { mutate, error: updateError } = useMutation<
    void,
    Error,
    { ids: GameVariant[]; newItems: GameVariantInfo[] },
    { previousGameVariants: GameVariantInfo[] | undefined }
  >({
    mutationFn: ({ ids }) => updateGameVariantOrder(ids),
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
    onError: (_error, _variables, context) => {
      if (context?.previousGameVariants) {
        queryClient.setQueryData(
          queryKeys.gameVariantsInfo(),
          context.previousGameVariants,
        );
      }
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.gameVariantsInfo(),
      });
    },
  });

  useEffect(() => {
    if (updateError) {
      onOrderUpdateErrorRef.current?.(updateError);
    }
  }, [updateError]);

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
    isError: !!error,
    error,
  };
}
