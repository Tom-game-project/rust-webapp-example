/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/*.{html, css}"],
  safelist: [
    "text-3xl",
    "font-bold",
    "underline",
    "rounded",
    "bg-blue-500",
    "bg-red-500",
    "text-white",
    "shadow-lg",
    // hover時の変化
    "hover:shadow-xl",
    "hover:bg-red-500",
    //
    "py-2",
    "px-4",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

