// https://nuxt.com/docs/api/configuration/nuxt-config
const vitePlugins = [];

export default defineNuxtConfig({
  modules: [
    '@pinia/nuxt',
    '@vueuse/nuxt',
    'nuxt-svgo',
    '@formkit/auto-animate/nuxt',
    '@nuxt/eslint',
    'v-gsap-nuxt',
  ],
  ssr: false,
  components: [
    '@/components',
    { path: '@/components/common', prefix: 'C' },
    { path: '@/components/animation', prefix: 'A' },
    { path: '@/components/ui', prefix: 'Ui' },
    { path: '@/components/cards', prefix: 'Card' },
    { path: '@/components/layout', prefix: 'L' },
  ],
  devtools: { enabled: false },
  css: ['reset-css', '@/assets/styles/base/index.scss'],
  ignore: ['**/src-tauri/**'],
  devServer: {
    host: '0',
  },
  features: { inlineStyles: false },
  compatibilityDate: '2025-07-15',
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: '@use "@/assets/styles/utils" as *;',
          silenceDeprecations: ['global-builtin', 'import'],
        },
      },
    },
    plugins: vitePlugins,
  },
  eslint: {
    config: {
      stylistic: true,
    },
  },
  svgo: { defaultImport: 'component', explicitImportsOnly: true },
});
