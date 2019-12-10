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
    template: `<app :rust="rust" :window="window" />`,
    computed: {
      rust() {
        return rust;
      },
      window() {
        return window;
      }
    }
  });
};
