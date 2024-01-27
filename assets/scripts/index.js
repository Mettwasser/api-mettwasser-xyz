"use strict";

import typewriter from "./typewriter.js";

typewriter("#main-heading", {
    text: "api.mettwasser.xyz",
    speed: 100,
    onFinish: () => typewriter("#sub-heading", {
        text: "Transparent and community-driven.",
        speed: 65,
    }),
});

