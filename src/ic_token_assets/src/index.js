import { ic_token } from "../../declarations/ic_token";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with ic_token actor, calling the greet method
  const greeting = await ic_token.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
