/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./templates/*.html", "./assets/**/*.js"],
    theme: {
        extend: {
            screens: {
                xs: "485px",
            },
            keyframes: {
                fadein: {
                    "0%": { opacity: 0 },
                    "100%": { opacity: 100 },
                },
            },
            animation: {
                fadein: "fadein 2s forwards",
            },
            dropShadow: {
                gray: "0 1px 2px rgba(243,244,246.1)",
            },
        },
    },
    plugins: [],
};
