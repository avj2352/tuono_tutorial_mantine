import type { TuonoConfig } from "tuono/config";
import path, { dirname } from "node:path";
import { fileURLToPath } from "node:url";

function getDirname(importMetaUrl: string): string {
  return dirname(fileURLToPath(importMetaUrl));
}

const __dirname = getDirname(import.meta.url);

const config: TuonoConfig = {
  vite: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "@tabler/icons-react": "@tabler/icons-react/dist/esm/icons/index.mjs",
    },
  },
};

export default config;
