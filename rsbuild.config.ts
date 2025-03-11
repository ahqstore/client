import { defineConfig } from '@rsbuild/core';
import { pluginReact } from "@rsbuild/plugin-react";

import path from "path"

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [pluginReact({
    fastRefresh: true
  })],
  html: {
    template: './index.html',
  },
  source: {
    entry: {
      index: './src/main.tsx',
    },
  },
  dev: {
    watchFiles: {
      paths: ["./src/**/*"],
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: host || undefined,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "@controls": path.resolve(__dirname, "./src/controls"),
    },
  },
});