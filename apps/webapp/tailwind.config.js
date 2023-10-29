/** @type {import('tailwindcss').Config} */

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
          fontFamily: {
            sans: ['Inter var', ...defaultTheme.fontFamily.sans],
          },
        },
      },
    plugins: [
      require('@tailwindcss/forms')
    ],
  }