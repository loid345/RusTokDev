/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./assets/**/*.css"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        rustok: {
          // Palette aligned with platform shadcn CSS vars:
          // --primary: 199 89% 48%  →  hsl(199, 89%, 48%) = #0ea5e9 (sky-500)
          // --secondary: 210 40% 96.1% →  #f0f4f8
          // --accent: 38 92% 50% → #f59e0b (amber-500)
          primary: "#0ea5e9",
          "primary-content": "#ffffff",
          secondary: "#f0f4f8",
          "secondary-content": "#1e293b",
          accent: "#f59e0b",
          "accent-content": "#000000",
          neutral: "#1e293b",
          "neutral-content": "#ffffff",
          "base-100": "#ffffff",
          "base-200": "#f8fafc",
          "base-300": "#e2e8f0",
          "base-content": "#0f172a",
          info: "#3abff8",
          success: "#22c55e",
          warning: "#f59e0b",
          error: "#ef4444",
        },
        rustok_dark: {
          // --primary dark: 199 89% 58% → hsl(199, 89%, 58%) ≈ #38bdf8 (sky-400)
          primary: "#38bdf8",
          "primary-content": "#0c1a2e",
          secondary: "#1e293b",
          "secondary-content": "#e2e8f0",
          accent: "#fbbf24",
          "accent-content": "#000000",
          neutral: "#334155",
          "neutral-content": "#ffffff",
          "base-100": "#0f172a",
          "base-200": "#1e293b",
          "base-300": "#334155",
          "base-content": "#f1f5f9",
          info: "#3abff8",
          success: "#22c55e",
          warning: "#f59e0b",
          error: "#f87171",
        },
      },
      "light",
      "dark",
    ],
    darkTheme: "rustok_dark",
  },
};
