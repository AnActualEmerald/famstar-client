import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
    plugins: [sveltekit()],
    
    server: {
        port: 1420,
        strictPort: true
    },

    envPrefix: ["VITE_", "TAURI_"]
});