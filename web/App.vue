<template>
  <div>
    <div>
      <label>Entry File Name</label>
      <label></label>
    </div>
    <div>
      <button @click="runJVM">run Rust JVM</button>
      <label class="uploadLabel" for="upload">
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
.uploadLabel {
  cursor: pointer;
  padding: 15px;
  margin: 0 10px 0 0;
  display: inline-block;
}
.uploadLabel input {
  display: none;
}
</style>
