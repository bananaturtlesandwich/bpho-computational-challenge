import init, { entry } from "./pkg/bpho_comp_challenge.js";

async function run() {
  await init();
  entry();
}

run();
