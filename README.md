# Tuono starter

This is the starter tuono project. To download it run in your terminal:

## Important Links
- [Mantine Framework](https://mantine.dev)
- [Layoutit app token generator](https://tokens.layoutit.com)
- [Layoutit main website](https://layoutit.com)
- [PicoCSS library](https://picocss.com/docs/nav)

# Tuono CLI commands

```shell
# tuono new project
$ tuono new [NAME]

# tuono start dev app
$ tuono dev
```

---

# Configure src alias `@` in Tuono

After a little more digging, I found that `alias` property exists in `tuono.config.ts`  > `TuonoConfig` type and I added this configuration.

I also has to install `npm -D @types/node`

```typescript
// ->> tuono.config.ts

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
    },
  },
};

export default config;
```

And this is the configuration to `tsconfig.json`

```json
/*->> tsconfig.json*/
/* src alias "@" configuration */
    "baseUrl": ".",
    "paths": {
      "@/*": [
        "src/*"
      ]
    },
```

Now I can use src alias using  `"@"` in my react components!!

---



