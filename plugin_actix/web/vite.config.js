import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from "@vitejs/plugin-vue-jsx"
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx()
  ],
  build:{
    outDir: "../static"
  },
  resolve: {
    // 配置路径别名
    alias: {
      '@': path.resolve(__dirname, './src')
    },
  },
  server:{    
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8088/', // 后台服务地址以及端口号
        changeOrigin: true, //是否跨域
        pathRewrite: { '^/api': '/' }
      }
    }
  }
})
