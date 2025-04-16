module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
    ],
  },
  theme: {
    extend: {
      colors: {
        "mycolor-1": "#F94892",
        "mycolor-2": "#FF7F3F",
        "mycolor-3": "#FBDF07",
        "mycolor-4": "#89CFFD",
      },
    },
  },
  variants: {},
  plugins: [],
};
