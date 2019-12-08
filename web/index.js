import Vue from 'vue';
import App from "./App.vue";

window.map = {};
window.onload = async () => {
  const rust = await import('./pkg');

  new Vue({
    el: "#app",
    components: {
      App,
    },
    data() {
      return {
        entryFileName: ""
      }
    },
    template: `<app :entry-file-name="entryFileName" :wasm-event="runWasm" />`,
    mounted() {
      const upload = document.querySelector("#upload");
      const self = this;
      upload.addEventListener("change", (e) => {
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
      });
    },
    methods: {
      runWasm(entryFileName) {
        rust.run_wasm(entryFileName);
      }
    }
  })
}
