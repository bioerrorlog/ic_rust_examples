import { sample_rust_dapp } from "../../declarations/counter_simple";

document.addEventListener('DOMContentLoaded', async function () {
  const counter = await counter_simple.get();
  document.getElementById("counter").innerText = "Counter: " + counter;
})

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const counter = await counter_simple.increment();
  document.getElementById("counter").innerText = "Counter: " + counter;
});