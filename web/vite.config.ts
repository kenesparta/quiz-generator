import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react-swc'

const config = {
  VITE_PORT: '3333'
}

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd())
  if (mode == "development") {
    return {
      plugins: [
        react()
      ],
      server: {
        host: '0.0.0.0',
        port: Number(env.VITE_PORT || config.VITE_PORT),
      },
    }
  }

  return {
    plugins: [
      react(),
    ],
    build: {
      rollupOptions: {}
    },
  }
})
