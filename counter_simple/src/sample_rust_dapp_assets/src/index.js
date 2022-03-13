import { sample_rust_dapp } from "../../declarations/sample_rust_dapp";

document.addEventListener('DOMContentLoaded', async function () {
  const counter = await sample_rust_dapp.get();
  document.getElementById("counter").innerText = "Counter: " + counter;
})

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const counter = await sample_rust_dapp.increment();
  document.getElementById("counter").innerText = "Counter: " + counter;
});