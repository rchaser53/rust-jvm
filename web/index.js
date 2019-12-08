import Vue from "vue";
import App from "./App.vue";

window.map = {};
window.onload = async () => {
  const rust = await import("./pkg");

  new Vue({
    el: "#app",
    components: {
      App
    },
    data() {
      return {
        entryFileName: ""
      };
    },
    template: `<app
  :entry-file-name="entryFileName"
  :upload-files="uploadFiles"
  :wasm-event="runWasm"
/>`,
    methods: {
      runWasm(entryFileName) {
        rust.run_wasm(entryFileName);
      },
      uploadFiles(e) {
        const self = this;
        const files = e.target.files;
        for (let file of files) {
          const fileName = file.name;
          const reader = new FileReader();
          reader.onload = (function(_) {
            return function(e) {
              self.entryFileName = fileName;
              window.map[fileName] = new Uint8Array(e.target.result);
            };
          })(file);
          reader.readAsArrayBuffer(file);
        }
      }
    }
  });
};
