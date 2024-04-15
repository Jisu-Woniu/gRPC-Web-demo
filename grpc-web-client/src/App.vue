<script setup lang="ts">
import SimpleRequest from "@/components/SimpleRequest.vue";
import StreamRequest from "@/components/StreamRequest.vue";
</script>

<template>
  <div class="box">
    <SimpleRequest />
  </div>
  <div class="box">
    <StreamRequest />
  </div>
</template>

<style scoped>
.box {
  margin: auto;
  border: 1px black solid;
  padding: 1em;
}
</style>
