import { scan } from "react-scan";
import React from "react";

scan({
  enabled: import.meta.env.DEV,
});

import ReactDOM from "react-dom/client";
import Router from "@/Router";
import "@/styles/global.css";
import Providers from "@/providers";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Providers>
      <Router />
    </Providers>
  </React.StrictMode>,
);
