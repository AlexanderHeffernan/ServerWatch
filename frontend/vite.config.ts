import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig({
    plugins: [vue(), VitePWA({
        filename: 'service-worker.js',
        injectRegister: false,
        manifest: false, // Keep the existing public/site.webmanifest and icons.
        workbox: {
            globPatterns: ['**/*.{js,css,html,png,svg,ico,webmanifest}'],
            cleanupOutdatedCaches: true,
            // API requests are cross-origin and must never be cached.
            navigateFallbackDenylist: [/^\/(metrics|test-connection|get-temp-config|set-temp-config|shutdown|reboot)$/],
        },
    })],
    build: { outDir: '../dist', emptyOutDir: true },
});
