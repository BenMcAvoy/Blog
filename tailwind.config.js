/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.{html,html.tera}"
  ],

  theme: {
    fontFamily: {
      'inter': ['Inter var', 'sans-serif'],
    },

    extend: {
      colors: {
        black: '#121212',
      }
    },
  },

  plugins: [],
}

