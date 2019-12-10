<template>
  <div class="frame">
    <label class="weight">Output:</label>
    <div class="output">
      <div v-for="(value, index) in output" :key="index">{{ value }}</div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    output: {
      type: Array
    }
  }
};
</script>

<style scoped>
.frame {
  display: flex;
}

.weight {
  font-weight: bold;
}

.output {
  margin-left: 30px;
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
