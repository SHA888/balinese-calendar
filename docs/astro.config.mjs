import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import tailwind from '@astrojs/tailwind';

export default defineConfig({
  site: 'https://sha888.github.io/balinese-calendar',
  base: '/balinese-calendar/',
  trailingSlash: 'never',
  integrations: [
    mdx(),
    tailwind({
      applyBaseStyles: false,
    }),
  ],
  markdown: {
    shikiConfig: {
      themes: {
        light: 'github-light',
        dark: 'github-dark',
      },
      langs: ['rust', 'javascript', 'toml', 'yaml'],
    },
  },
  vite: {
    optimizeDeps: {
      exclude: ['pagefind'],
    },
  },
});
