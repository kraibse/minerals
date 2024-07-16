/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'active': '#4285F4',
        'active-mute': '#4286f4',
        'primary': '#23272e',
        'alternate': '#1e2227',
        'highlighted': '#2d313b',
      },
    },
  },
  plugins: [],
}

