import { useGameVariants } from "@/hooks/useGameVariants";
import PlayTimeChart from "./components/PlayTimeChart";

function AchievementsPage() {
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

  return (
    <main className="p-8">
      <PlayTimeChart variants={gameVariants} />
    </main>
  );
}

export default AchievementsPage;
