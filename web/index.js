window.onload = async () => {
  const rust = await import('./pkg');
  const button = document.querySelector("#emitButton");
  const upload = document.querySelector("#upload");
  let result = "";
  upload.addEventListener("change", (e) => {
    const files = e.target.files;
    for (file of files) {
      const reader = new FileReader();
      reader.onload = (function(theFile) {
        return function(e) {
          result = new Uint8Array(e.target.result);
        };
      })(file);
      reader.readAsArrayBuffer(file);
    }
  });

  button.addEventListener("click", () => {
    rust.greet("Test.class", result);
  })
}