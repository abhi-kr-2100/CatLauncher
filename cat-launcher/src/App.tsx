import { BrowserRouter, Route, Routes } from "react-router-dom";

import NavBar from "@/components/NavBar";
import { routes } from "@/routes";

/**
 * The root component of the CatLauncher application.
 * Defines the overall layout and handles client-side routing.
 *
 * @returns The rendered application.
 */
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
                element={
                  route.layout === "sidebar" ? (
                    route.element
                  ) : (
                    <div className="flex-1 overflow-y-auto p-2 flex flex-col">
                      {route.element}
                    </div>
                  )
                }
              />
            ))}
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
