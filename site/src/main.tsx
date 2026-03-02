import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { Analytics } from "@vercel/analytics/react";
import App from "./app/App";
import "./index.css";

const basePath = import.meta.env.VITE_BASE_PATH ?? "";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter basename={basePath}>
      <App />
      <Analytics />
    </BrowserRouter>
  </StrictMode>
);
