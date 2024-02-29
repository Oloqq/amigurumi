import Plushie from './plushie';
import "./lib/interaction";

import * as simulator from "./lib/simulation";

function advance() {
  simulator.send("advance");
}

const customGui = {
  edgesVisible: true
}

const pattern = document.getElementById("pattern");
const updateButton = document.getElementById("update");
const status = document.getElementById("status");

updateButton.addEventListener("click", () => {
  let text = pattern.value;
  simulator.send(`pattern ${text}`);
});

function main() {
  const app = simulator.init();
  const gui = app.gui;

  const simulationWorld = new Plushie(status);

  gui.add({ advance }, 'advance').name("Advance 1 step");
  gui.add(customGui, 'edgesVisible').name("Display edges (expensive)").onChange((_value) => {
    simulationWorld.toggleLinks();
  });

  simulator.connect("ws://127.0.0.1:8080", simulationWorld);
}

main();