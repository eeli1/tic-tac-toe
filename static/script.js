import init, { greet } from "../pkg/tic_tac_toe.js";
async function run() {
    await init();
    greet();
}
run();