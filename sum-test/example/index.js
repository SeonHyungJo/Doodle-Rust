// wasm test 1000000 => 0.39013671875ms
const js = import("../pkg/hello_wasm.js");

// js test 10000000 => 19.285000002128072ms
// const startTime = performance.now();
// let maxNum = 10000000
// let check = 0;

// for (let i = 0; i < maxNum; i++) {
//     check ++;
// }

// const endTime = performance.now()
// console.log("JS Time : ", endTime - startTime, check)
