import path from "node:path";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig, type PluginOption } from "vite";

const host = process.env.TAURI_DEV_HOST;

/** @type {import('vite').UserConfig} */
export default defineConfig(() => ({
	plugins: [...react(), tailwindcss() as unknown as PluginOption],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			ignored: ["**/src-tauri/**"],
		},
	},
	build: {
		target:
			process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
		sourcemap: !!process.env.TAURI_ENV_DEBUG,
	},
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},
	envPrefix: ["VITE_", "TAURI_ENV_*"],
}));
