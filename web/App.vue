<template>
  <div>
    <div class="appHeader">
      <label :class="runJVMButtonClass" for="runJVM">
        Run Rust JVM
        <input type="button" id="runJVM" :disabled="!canRunJVM" @click="runJVM">
      </label>
      <div style="margin: 2px;">
        <label>Entry File Name:</label>
        <label>{{ entryFileName }}</label>
      </div>
    </div>
    <div>
      <label class="labelButton" for="upload">
        Select File
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
</template>

<script>
export default {
  props: {
    entryFileName: {
      type: String,
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
      return this.canRunJVM
        ? "labelButton"
        : "labelButton disable";
    }
  },
  methods: {
    uploads(e) {
      this.uploadFiles(e);
    },
    runJVM() {
      this.wasmEvent(this.entryFileName);
    }
  }
};
</script>

<style scoped>
.appHeader {
  display: flex;
}

.labelButton {
  cursor: pointer;
  border: solid 0.5px #9E9E9E;
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
  background-color: #AAA;
  color: #DDD;
}
</style>
