import('./pkg')
  .catch(console.error);

const CANVAS_ID = "canvas";
const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

function resizeCanvas() {
  currentImageData = context.getImageData(0, 0, canvas.width, canvas.height);

  // Set internal canvas resolution size
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  if (currentImageData) {
    context.putImageData(currentImageData, 0, 0);
  }
}

// Resize canvas when the window is resized
window.addEventListener("resize", resizeCanvas);

// Initial resize
resizeCanvas();
