import { createBrowserRouter, RouterProvider } from "react-router-dom";
import App from "./App";
import { ROUTES } from "./lib/routes";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: ROUTES.map((route) => ({
      path: route.path,
      element: <route.component />,
    })),
  },
]);

const Router = () => {
  return <RouterProvider router={router} />;
};

export default Router;
