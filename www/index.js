import * as sim from "lib-simulation-wasm";

function drawTriangle(ctxt, x, y, size, rotation) {
  ctxt.beginPath();
  ctxt.moveTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5,
  );
  ctxt.lineTo(
    x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
    y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
  );
  ctxt.lineTo(
    x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
    y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
  );
  ctxt.lineTo(
    x - Math.sin(rotation) * size * 1.5,
    y + Math.cos(rotation) * size * 1.5,
  );
  ctxt.stroke();
}

const simulation = new sim.Simulation();
const world = simulation.world();
console.log(world);

const viewport = document.getElementById('viewport');
const viewportHeight = viewport.clientHeight;
const viewportWidth = viewport.clientWidth;
const viewportScale = window.devicePixelRatio || 1;

viewport.height = viewportHeight * viewportScale;
viewport.width = viewportWidth * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext('2d');
ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(0,0,0)';

for (const animal of simulation.world().animals) {
  drawTriangle(
    ctxt,
    animal.x * viewportWidth,
    animal.y * viewportHeight,
    0.01 * viewportWidth,
    animal.rotation);
}
