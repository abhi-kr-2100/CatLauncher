import { useQuery } from "@tanstack/react-query";
import { fetchGameVariantsInfo } from "@/lib/utils";
import GameVariant from "@/GameVariant";

function App() {
  const {
    data: gameVariantsInfo = [],
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useQuery({
    queryKey: ["gameVariantsInfo"],
    queryFn: fetchGameVariantsInfo,
  });

  if (gameVariantsLoading) {
    return <p>Loading...</p>;
  }

  if (gameVariantsError) {
    return <p>Error: {gameVariantsErrorObj?.message ?? "Unknown error"}</p>;
  }

  return (
    <main className="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-2 p-2">
      {gameVariantsInfo.map((variant) => (
        <GameVariant key={variant.name} variant={variant} />
      ))}
    </main>
  );
}

export default App;
