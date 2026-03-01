/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        rust: {
          50: '#fef3e2',
          100: '#fde4b9',
          200: '#fbc97a',
          300: '#f9a83b',
          400: '#f7931e',
          500: '#e67e22',
          600: '#c0601a',
          700: '#9a4614',
          800: '#7a370f',
          900: '#5a280a',
        },
        crab: {
          50: '#fff1f0',
          100: '#ffe0de',
          200: '#ffc7c2',
          300: '#ff9e96',
          400: '#ff6b5b',
          500: '#ff3e2e',
          600: '#ed2010',
          700: '#c81608',
          800: '#a5160c',
          900: '#881a12',
        },
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
}
