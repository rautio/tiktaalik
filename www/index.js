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
  ctxt.fillStyle = 'rgb(255,255,255)';
  ctxt.fill();
  ctxt.stroke();
}

function drawCircle(ctxt, x, y, radius) {
  ctxt.beginPath();
  ctxt.arc(x, y, radius, 0, 2.0 * Math.PI);

  ctxt.fillStyle = 'rgb(0,255,128)';
  ctxt.fill();
}

const simulation = new sim.Simulation();

document.getElementById("train").onclick = function() {
  console.log(simulation.train());
}

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


function redraw() {
  ctxt.clearRect(0,0, viewportWidth, viewportHeight);

  simulation.step();

  const world = simulation.world();

  for (const food of world.foods) {
    drawCircle(
      ctxt,
      food.x * viewportWidth,
      food.y * viewportHeight,
      (0.01 / 2.0) * viewportWidth,
    )
  }

  for (const animal of world.animals) {
    drawTriangle(
      ctxt,
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation);
  }
  requestAnimationFrame(redraw);
}

redraw();
