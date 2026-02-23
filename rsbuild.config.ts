import { defineConfig } from '@rsbuild/core';

import { pluginBabel } from '@rsbuild/plugin-babel';
import { pluginReact } from "@rsbuild/plugin-react";

import path from "path"

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [
    pluginReact(),
    pluginBabel({
      include: /\.(?:jsx|tsx)$/,
      babelLoaderOptions(opts) {
        opts.plugins?.unshift('babel-plugin-react-compiler');
      },
    }),
  ],
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
  tools: {
    rspack: {
      watchOptions: {
        ignored: /^(?!.*\/src\/).*/
      }
    }
  }
});