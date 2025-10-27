import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import GameVariantCard from "@/PlayPage/GameVariantCard";
import { fetchGameVariantsInfo } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import Spinner from "@/components/ui/spinner";

function PlayPage() {
  const {
    data: gameVariantsInfo = [],
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useQuery({
    queryKey: queryKeys.gameVariantsInfo,
    queryFn: fetchGameVariantsInfo,
  });

  useEffect(() => {
    if (gameVariantsError) {
      toastCL("error", "Failed to fetch game variants.", gameVariantsErrorObj);
    }
  }, [gameVariantsError, gameVariantsErrorObj]);

  if (gameVariantsLoading) {
    return (
      <div className="flex justify-center items-center h-full">
        <Spinner />
      </div>
    );
  }

  if (gameVariantsError) {
    return null;
  }

  return (
    <main className="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-2 p-2">
      {gameVariantsInfo.map((variantInfo) => (
        <GameVariantCard key={variantInfo.id} variantInfo={variantInfo} />
      ))}
    </main>
  );
}

export default PlayPage;
