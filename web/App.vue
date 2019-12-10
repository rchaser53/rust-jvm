<template>
  <div class="frame">
    <setting
      :clear-output="clearOutput"
      :entry-file-name="entryFileName"
      :selected-file-names="fileNames"
      :upload-files="uploadFiles"
      :upadate-entry-file-name="upadateEntryFileName"
      :wasm-event="runWasm"
    />
    <result :output="output" />
  </div>
</template>

<script>
import Result from "./Result.vue";
import Setting from "./Setting.vue";

export default {
  components: {
    Setting,
    Result
  },
  props: {
    rust: {
      required: true
    },
    window: {
      required: true
    }
  },
  data() {
    return {
      entryFileName: "",
      fileNames: [],
      output: []
    };
  },
  methods: {
    clearOutput() {
      this.output = [];
    },
    runWasm(entryFileName) {
      this.rust.run_wasm(entryFileName);
      this.output = this.window.output.slice(0);
    },
    upadateEntryFileName(fileName) {
      this.entryFileName = fileName;
    },
    uploadFiles(e) {
      const files = e.target.files;
      const self = this;
      for (let file of files) {
        const fileName = file.name;
        this.entryFileName = fileName;
        if (!this.fileNames.includes(fileName)) {
          this.fileNames.push(fileName);
        }

        const reader = new FileReader();
        reader.onload = (function(_) {
          return function(e) {
            self.window.map[fileName] = new Uint8Array(e.target.result);
          };
        })(file);
        reader.readAsArrayBuffer(file);
      }
    }
  }
};
</script>

<style scoped>
.frame {
  display: flex;
}

.settingHeader {
  display: flex;
}

.labelButton {
  cursor: pointer;
  border: solid 0.5px #9e9e9e;
  padding: 2px 4px;
  margin: 2px;
  line-height: 2em;
  border-radius: 4px;
  font: 400 11px system-ui;
}
.labelButton input {
  display: none;
}

.disable {
  background-color: #aaa;
  color: #ddd;
}

.weight {
  font-weight: bold;
}

.fileNameList {
  margin: 0px;
  cursor: pointer;
}

.fileNameList > li:hover {
  opacity: 0.5;
  color: blue;
}
</style>
