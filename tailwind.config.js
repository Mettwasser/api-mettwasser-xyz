/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./templates/*.html", "./assets/**/*.js"],
    theme: {
        extend: {
            keyframes: {
                fadein: {
                    "0%": { opacity: 0, backgroundColor: "#000000" },
                    "100%": { opacity: 100, backgroundColor: "#262626" },
                },
            },
            animation: {
                fadein: "fadein 2s forwards",
            },
        },
    },
    plugins: [],
};
