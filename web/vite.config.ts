import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  server:{
    port:7070,
    proxy:{
      '/api': 'http://localhost:8080',
    }
  },
  plugins: [vue()],
})
