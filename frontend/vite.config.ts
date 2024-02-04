import { resolve } from "path";
import { defineConfig } from "vite";
import { name, dependencies } from "./package.json";
import dts from "vite-plugin-dts";

export default defineConfig(({ command, mode }) => {
  console.log(command, mode);

  switch (command) {
    case "serve":
      return {
        server: {
          host: true,
          port: 666,
        },
        build: {
          outDir: "../docs",
          emptyOutDir: true,
        },
        preview: {
          port: 666,
        },
        root: "./src",
      };
    case "build":
      if (mode === "tests" || mode === "production") {
        return {
          root: "./cypress/pages",
          build: {
            outDir: "./dist",
          },
          base: "",
        };
      } else {
        return {
          root: "./src",
          build: {
            outDir: "../docs",
            emptyOutDir: true,
          },
          base: "",
        };
      }
  }
});
