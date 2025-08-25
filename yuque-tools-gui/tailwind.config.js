/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      zIndex: {
        dropdown: '100',
        sticky: '200',
        fixed: '300',
        modal: '1000',
        popover: '1100',
        tooltip: '1200',
        toast: '1300',
        dialog: '9999',
      },
    },
  },
  plugins: [],
}
