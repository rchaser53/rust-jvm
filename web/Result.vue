<template>
  <div>
    <label class="weight">Output:</label>
  </div>
</template>

<script>
export default {
  props: {}
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
