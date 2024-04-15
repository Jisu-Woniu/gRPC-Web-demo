<script setup lang="ts">
import SimpleRequest from "@/components/SimpleRequest.vue";
import StreamRequest from "@/components/StreamRequest.vue";
import { VueQueryDevtools } from "@tanstack/vue-query-devtools";
</script>

<template>
  <div class="box">
    <SimpleRequest />
  </div>
  <div class="box">
    <StreamRequest />
  </div>
  <VueQueryDevtools />
</template>

<style scoped>
.box {
  margin: 1em auto;
  border: 1px black solid;
  padding: 1em;
}
</style>
