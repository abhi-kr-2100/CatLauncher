import { useMutation, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { applySettings } from "@/lib/commands";
import type { ThemeColors } from "@/generated-types/ThemeColors";

export function useApplySettings(
  onError?: (err: Error) => void,
  onSuccess?: () => void,
) {
  const queryClient = useQueryClient();

  const { mutate: applySettingsFn, isPending } = useMutation({
    mutationFn: async (params: {
      maxBackups: number;
      parallelRequests: number;
      fontLocation?: string;
      themeColors?: ThemeColors;
    }) => {
      await applySettings(
        params.maxBackups,
        params.parallelRequests,
        params.fontLocation,
        params.themeColors,
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.settings(),
      });
      onSuccess?.();
    },
    onError: (err) => {
      onError?.(err as Error);
    },
  });

  return { applySettings: applySettingsFn, isPending };
}
