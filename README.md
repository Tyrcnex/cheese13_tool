# whats this project?

this is a 10L cheese simulator, with maps that can be completed in 13 pieces or less. i think due to a slight build error this is actually going to be 12 most of the time, but wtv

currently you have to mess with the dev console if you want to change config like key handling or movement settings. change it like this for example:

```js
const myKeys = {
    moveLeft: ["KeyK"],
    moveRight: ["KeyP"],
    rotateCW: ["KeyO"],
    rotateCCW: ["KeyD"],
    rotate180: ["KeyS"],
    softDrop: ["KeyL"],
    hardDrop: ["Space"],
    hold: ["KeyA"],
    retry: ["KeyR"],
    newMap: ["KeyT"]
};
const myHandling = {
    das: 60,
    arr: 0,
    sdr: 20 // soft drop rate in ms
}
localStorage.setItem("KEYS", JSON.stringify(myKeys));
localStorage.setItem("HANDLING", JSON.stringify(myHandling));
```

default keys are defined at the bottom of `data.js`. have a look yourself.

the maps are also pregenerated, so theres only about 5000 or so maps to choose from. i will fix this in the future.