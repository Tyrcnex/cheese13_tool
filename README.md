# whats this project?

this is a 10L cheese simulator, with maps that can be completed in 13 pieces or less.

currently there's no support for retrying the same map (you have to reload), and you have to mess with the dev console if you want to change config like key handling or movement settings. change it like this for example:

```js
const myKeys = {
    moveLeft: ["KeyK"],
    moveRight: ["KeyP"],
    rotateCW: ["KeyO"],
    rotateCCW: ["KeyD"],
    rotate180: ["KeyS"],
    softDrop: ["KeyL"],
    hardDrop: ["Space"],
    hold: ["KeyA"]
};
const myHandling = {
    das: 60,
    arr: 0,
    sdr: 20 // soft drop rate in ms
}
localStorage.setItem("KEYS", JSON.stringify(myKeys));
localStorage.setItem("HANDLING", JSON.stringify(myHandling));
```

the maps are also pregenerated, so theres only about 800 or so maps to choose from. i will fix this in the future.