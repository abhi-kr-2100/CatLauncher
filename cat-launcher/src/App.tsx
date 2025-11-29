import { BrowserRouter, Route, Routes } from "react-router-dom";
import PlayPage from "@/pages/PlayPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<PlayPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
