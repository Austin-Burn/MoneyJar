import tailwindcss from "@tailwindcss/vite";
import viteReact from "@vitejs/plugin-react";
import { defineConfig } from "vite";

import { resolve } from "node:path";
import { TanStackRouterVite } from "@tanstack/router-plugin/vite";
import vitePluginSvgr from "vite-plugin-svgr";

const ReactCompilerConfig = {

}

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		TanStackRouterVite({ autoCodeSplitting: true }),
		viteReact({
			babel: {
				plugins: [["babel-plugin-react-compiler", ReactCompilerConfig]]
			}
		}),
		tailwindcss(),
		vitePluginSvgr()
	],
	test: {
		globals: true,
		environment: "jsdom",
	},
	resolve: {
		alias: {
			"@": resolve(__dirname, "./src"),
		},
	},
	server: {
		proxy: {
			"/api": "http://localhost:2000",
		}
	}
});
