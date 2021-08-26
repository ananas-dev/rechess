const colors = require("tailwindcss/colors");

module.exports = {
  mode: "jit",
  purge: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extends: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
