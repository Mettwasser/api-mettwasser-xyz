"use strict";

import typewriter from "./typewriter.js";

const endpoints = [
    "/image/round?url=https://cdn.discordapp.com/avatars/350749990681051149/9beb858df8c21c31279c6dfc9f2ccd68.png?size=1024&auto=true",
];
const randomEndpoint = endpoints[Math.floor(Math.random() * endpoints.length)];

typewriter("#heading", {
    text: "api.mettwasser.xyz",
    speed: 100,
});

document.querySelector("#randomEndpoint").setAttribute("href", randomEndpoint);
