import init, * as eventHandlers from "./sketch.js";

let state = null;
for (const key in eventHandlers) {
    const func = eventHandlers[key];
    globalThis[key] = function () {
        if (state === null) return
        func(state);
    }
}

globalThis.setup = async function() {
    await init();
    state = eventHandlers.setup();
}