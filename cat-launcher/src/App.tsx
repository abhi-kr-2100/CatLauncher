import { BrowserRouter, Route, Routes } from "react-router-dom";
import NavBar from "@/components/NavBar";
import { routes } from "@/routes";

function App() {
  return (
    <BrowserRouter>
      <NavBar />
      <div className="p-2">
        <Routes>
          {routes.map((route) => (
            <Route key={route.path} path={route.path} element={route.element} />
          ))}
        </Routes>
      </div>
    </BrowserRouter>
  );
}

export default App;
