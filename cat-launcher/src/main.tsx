import { scan } from "react-scan";
import React from "react";

scan({
  enabled: import.meta.env.DEV,
});

import ReactDOM from "react-dom/client";
import App from "@/App";
import "@/styles/global.css";
import Providers from "@/providers";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Providers>
      <App />
    </Providers>
  </React.StrictMode>,
);
