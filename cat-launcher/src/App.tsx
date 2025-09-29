import { useQuery } from "@tanstack/react-query";
import "./App.css";
import { fetchGameVariantsInfo } from "./utils";
import GameVariant from "./GameVariant";


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
    <main className="container">
      {gameVariantsInfo.map((variant) => (
        <GameVariant key={variant.name} variant={variant} />
      ))}
    </main>
  );
}

export default App;
