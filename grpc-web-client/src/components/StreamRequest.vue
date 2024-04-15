<script setup lang="ts">
import { useTransport } from "@/useTransport";
import { EchoService } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_es/echo/v1/echo_connect";
import { createPromiseClient } from "@connectrpc/connect";
import { ref } from "vue";
const transport = useTransport();

const message = ref("");

const responses = ref<string[]>([]);

const client = createPromiseClient(EchoService, transport);

const onSubmit = async () => {
  responses.value = [];
  const stream = client.echoStream({
    message: message.value,
    times: 5,
  });

  for await (const item of stream) {
    responses.value.push(item.message);
  }
};
</script>
<template>
  <form @submit.prevent="onSubmit">
    <input v-model="message" />
    <button>发送</button>
  </form>
  <p>Responses from server:</p>
  <ol>
    <li v-for="response in responses" :key="response">{{ response }}</li>
  </ol>
</template>
<style scoped></style>
