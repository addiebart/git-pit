import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  root: './src/web',
  publicDir: false,
  build: {
    outDir: '../../build/web',
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, 'src/web/html/index.html'),
    },
  },
  server: {
    host: '0.0.0.0',
    port: 8080,
    watch: {
      usePolling: true,
    },
  },
});