import {
  render
} from './pkg';

console.log("#[wasm_bindgen(start)] completed");

if (false) {
  console.log("Repeating render under raf(), this is of course incredibly slow")
  console.log("but using raf() is necessary for Spector.js to capture WebGL state");

  const renderLoop = () => {
    render();
    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
}