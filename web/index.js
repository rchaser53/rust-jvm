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
        entryFileName: "",
        fileNames: []
      };
    },
    template: `<app
  :entry-file-name="entryFileName"
  :selected-file-names="fileNames"
  :upload-files="uploadFiles"
  :upadate-entry-file-name="upadateEntryFileName"
  :wasm-event="runWasm"
/>`,
    methods: {
      runWasm(entryFileName) {
        rust.run_wasm(entryFileName);
      },
      upadateEntryFileName(fileName) {
        this.entryFileName = fileName;
      },
      uploadFiles(e) {
        const files = e.target.files;
        for (let file of files) {
          const fileName = file.name;
          this.entryFileName = fileName;
          if (!this.fileNames.includes(fileName)) {
            this.fileNames.push(fileName);
          }

          const reader = new FileReader();
          reader.onload = (function(_) {
            return function(e) {
              window.map[fileName] = new Uint8Array(e.target.result);
            };
          })(file);
          reader.readAsArrayBuffer(file);
        }
      }
    }
  });
};
