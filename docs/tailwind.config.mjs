/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      fontFamily: {
        display: ['Fraunces', 'serif'],
        body: ['DM Sans', 'sans-serif'],
        mono: ['Geist Mono', 'monospace'],
      },
      colors: {
        balinese: {
          gold: '#D4AF37',
          red: '#8B0000',
          turquoise: '#40E0D0',
          'dark-wood': '#2C1810',
          'light-stone': '#F5F1E8',
          'warm-white': '#FEFDFB',
        },
        text: {
          primary: '#1A1A1A',
          secondary: '#5A5A5A',
        },
      },
      fontVariationSettings: {
        'wonk-display': "'WONK' 1, 'opsz' 144",
      },
      animation: {
        'fade-in': 'fadeIn 0.6s ease-out',
        'slide-up': 'slideUp 0.4s ease-out',
        'gold-shimmer': 'goldShimmer 3s ease-in-out infinite',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        goldShimmer: {
          '0%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
          '100%': { backgroundPosition: '0% 50%' },
        },
      },
      backgroundImage: {
        'balinese-pattern': "url('data:image/svg+xml,%3Csvg width=\"100\" height=\"100\" xmlns=\"http://www.w3.org/2000/svg\"%3E%3Cpath d=\"M0 50h100M50 0v100\" stroke=\"%23D4AF37\" stroke-width=\"0.5\" opacity=\"0.3\"/%3E%3C/svg%3E')",
        'temple-gate': "url('data:image/svg+xml,%3Csvg width=\"120\" height=\"60\" xmlns=\"http://www.w3.org/2000/svg\"%3E%3Cpath d=\"M10 0v60M110 0v60M10 20h30M80 20h30M10 40h30M80 40h30\" stroke=\"%23D4AF37\" stroke-width=\"2\" fill=\"none\"/%3E%3C/svg%3E')",
      },
    },
  },
  plugins: [
    // require('@tailwindcss/typography'),
  ],
}
