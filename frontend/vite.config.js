import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: {
    // during `npm run dev`, forward API calls to the backend so you don't
    // need to run the Rust build just to iterate on the UI
    proxy: {
      "/api": "http://localhost:3000",
    },
  },
});
