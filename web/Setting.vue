<template>
  <div style="diplay: block; width: 300px;">
    <div class="settingHeader">
      <label :class="runJVMButtonClass" for="runJVM">
        Run Rust JVM
        <input
          type="button"
          id="runJVM"
          :disabled="!canRunJVM"
          @click="runJVM"
        />
      </label>
      <div>
        <label class="labelButton" for="upload">
          Upload File
          <input
            type="file"
            id="upload"
            @change="uploads"
            multiple="multiple"
            accept=".class"
          />
        </label>
      </div>
    </div>
    <div>
      <div>
        <label class="weight">Entry File Name:</label>
        <label>{{ entryFileName }}</label>
      </div>
      <div>
        <label class="weight">Uploaded Class Files:</label>
        <ul class="fileNameList">
          <li
            v-for="fileName in selectedFileNames"
            :key="fileName"
            @click="selectEntryFileName(fileName)"
          >
            {{ fileName }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    entryFileName: {
      type: String,
      required: true
    },
    selectedFileNames: {
      type: Array,
      required: true
    },
    upadateEntryFileName: {
      type: Function,
      required: true
    },
    uploadFiles: {
      type: Function,
      required: true
    },
    wasmEvent: {
      type: Function,
      required: true
    }
  },
  computed: {
    canRunJVM() {
      return this.entryFileName !== "";
    },
    runJVMButtonClass() {
      return this.canRunJVM ? "labelButton" : "labelButton disable";
    }
  },
  methods: {
    runJVM() {
      this.wasmEvent(this.entryFileName);
    },
    selectEntryFileName(value) {
      this.upadateEntryFileName(value);
    },
    uploads(e) {
      this.uploadFiles(e);
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
