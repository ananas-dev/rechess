const colors = require("tailwindcss/colors");

module.exports = {
  mode: "jit",
  purge: ["./public/index.html", "./src/**/*.svelte"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extends: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
