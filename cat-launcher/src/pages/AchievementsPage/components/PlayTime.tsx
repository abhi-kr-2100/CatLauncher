import { useGameVariants } from "@/hooks/useGameVariants";
import PlayTimeChart from "./PlayTimeChart";

export default function PlayTime() {
  const {
    gameVariants,
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useGameVariants();

  if (gameVariantsLoading) {
    return <p>Loading...</p>;
  }

  if (gameVariantsError) {
    return (
      <p>Error: {gameVariantsErrorObj?.message ?? "Unknown error"}</p>
    );
  }

  return <PlayTimeChart variants={gameVariants} />;
}
