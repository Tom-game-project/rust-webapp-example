/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/*.{html, css}"],
  safelist: [
    "underline",
    "text-3xl",
    "text-lg",
    "font-bold",
    "font-semibold",
    "mb-4",
    // round
    "rounded",
    "rounded-lg",
    "space-x-1",
    "space-x-6",
    "space-y-1",
    // block
    "flex",
    "justify-between",
    "block",
    "inline-block",
    "bg-white",
    "bg-gray-100",
    "bg-green-100",
    "bg-blue-500",
    "bg-red-500",
    "text-white",
    "shadow-md",
    "shadow-lg",
    // hover時の変化
    "hover:shadow-xl",
    "hover:bg-red-500",
    //
    "p-6",
    "p-10",
    //
    "py-2",
    "px-4",
    "px-2",
    // divide
    "divide-y",
    "divide-gray-300",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

