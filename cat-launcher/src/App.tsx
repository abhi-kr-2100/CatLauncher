import { BrowserRouter, Route, Routes } from "react-router-dom";

import NavBar from "@/components/NavBar";
import { routes } from "@/routes";

function App() {
  return (
    <BrowserRouter>
      <div className="flex h-screen flex-col overflow-hidden">
        <NavBar />
        <main className="flex-1 overflow-hidden flex flex-col">
          <Routes>
            {routes.map((route) => (
              <Route
                key={route.path}
                path={route.path}
                element={route.element}
              />
            ))}
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
