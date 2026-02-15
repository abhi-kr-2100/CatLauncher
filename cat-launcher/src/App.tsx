import { BrowserRouter, Route, Routes } from "react-router-dom";

import NavBar from "@/components/NavBar";
import { DefaultWrapper, routes } from "@/routes";

function App() {
  return (
    <BrowserRouter>
      <div className="flex h-screen flex-col">
        <NavBar />
        <main className="flex flex-1 flex-col overflow-hidden">
          <Routes>
            {routes.map((route) => {
              const Wrapper = route.customWrapper ?? DefaultWrapper;

              return (
                <Route
                  key={route.path}
                  path={route.path}
                  element={<Wrapper>{route.element}</Wrapper>}
                />
              );
            })}
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
