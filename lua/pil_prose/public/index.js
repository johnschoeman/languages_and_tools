const clipboard = new ClipboardJS("pre, code", {
  text: function (trigger) {
    const value = trigger.innerHTML.trim();
    return value;
  },
});

clipboard.on("success", () => {
  const div = document.createElement("div");
  div.append("copied!");
  div.id = "copied-flash";
  div.classList.add("flash-message");
  document.body.appendChild(div);

  setTimeout(() => {
    const div = document.getElementById("copied-flash");
    div.remove();
  }, 1500);
});
