import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { remix } from "@remix-run/dev";

export default defineConfig({
  plugins: [remix(), react()],
  server: {
    port: 3001,
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
});

