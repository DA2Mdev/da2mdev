module.exports = {
  content: [
    "index.html",
    "./src/*.rs",
    "./styles/*.css"
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwinwcss/forms"), require("@tailwindcss/typography")],
}
