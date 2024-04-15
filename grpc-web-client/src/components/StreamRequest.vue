<script setup lang="ts">
import { useTransport } from "@/useTransport";
import { EchoService } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_es/echo/v1/echo_connect";
import { createPromiseClient } from "@connectrpc/connect";
import { ref } from "vue";
const transport = useTransport();

const message = ref("");
const times = ref(5);

const responses = ref<string[]>([]);

const client = createPromiseClient(EchoService, transport);

const onSubmit = async () => {
  responses.value = [];
  const stream = client.echoStream({
    message: message.value,
    times: times.value,
  });

  for await (const item of stream) {
    responses.value.push(item.message);
  }
};
</script>
<template>
  <form @submit.prevent="onSubmit">
    <label for="message">Message</label>
    <input id="message" v-model="message" />

    <label for="times">Times</label>
    <input id="times" v-model="times" type="number" />

    <button>Send</button>
  </form>
  <p>Responses from server:</p>
  <ol>
    <li v-for="response in responses" :key="response">{{ response }}</li>
  </ol>
</template>
<style scoped>
form {
  display: flex;
  flex-direction: column;
}

form > * {
  margin: 0.5em auto;
}
</style>
