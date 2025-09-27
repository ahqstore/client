import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: ['./src/**.ts'],
  dts: true,
  clean: true,
  treeshake: false,
  format: ["cjs", "esm"],
  platform: "browser"
})