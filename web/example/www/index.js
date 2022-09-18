import * as wasm from "example-tree";

wasm.init_tree();

const counterContainer = document.getElementById("counter");
const counter = document.createElement("div");

const counterTitle = document.createElement("h3");
counterTitle.innerText = "Count";

const counterButton = document.createElement("button");
counterButton.onclick = _ => console.log("clicked");
counterButton.innerText = "Click me";

counter.appendChild(counterTitle);
counter.appendChild(counterButton);

counterContainer.appendChild(counter);
