# WASM Canvas Floodfill

```javascript
import init, { floodfill } from "./wasm_canvas_floodfill.js";

init().then(() => {
  let context = canvas.getContext("2d");
  const newData = floodfill(
    context.getImageData(0, 0, canvas.width, canvas.height).data,
    [startX, startY],
    canvas.width,
    canvas.height,
    color
  );
  const imgData = new ImageData(newData, canvas.width, canvas.height);
  context.putImageData(imgData, 0, 0);
})
```
