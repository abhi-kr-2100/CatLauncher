import React from "react";
import ReactDOM from "react-dom/client";
import App from "@/App";
import Providers from "@/Providers";
import "@/styles/global.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Providers>
      <App />
    </Providers>
  </React.StrictMode>,
);
