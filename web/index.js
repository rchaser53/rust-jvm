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
          const value = e.target.result;
          result = atob(value.slice(value.indexOf(',') + 1));
        };
      })(file);
      reader.readAsDataURL(file);
    }
  });
  
  button.addEventListener("click", () => {
    rust.greet(result);
  })
}