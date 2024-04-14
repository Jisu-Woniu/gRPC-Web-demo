<script setup lang="ts">
import { echo } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_query-es/echo/v1/echo-EchoService_connectquery";
import { greet } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_query-es/greet/v1/greet-GreetService_connectquery";
import { useMutation } from "./useMutation";
import { ref } from "vue";

const input = ref<string>();

const echoMutation = useMutation(echo);
const greetMutation = useMutation(greet);

const name = echoMutation.data;
const greeting = greetMutation.data;

const onclick = () => {
  echoMutation.mutate({ message: input.value });
  greetMutation.mutate({ name: input.value });
};
</script>

<template>
  <form>
    <label for="name">Your name: </label>
    <input id="name" v-model="input" />
    <button type="button" @click="onclick">send</button>
  </form>

  <p>Your name is: {{ name?.message }}</p>
  <p>Greet from server: {{ greeting?.greeting }}</p>
</template>

<style scoped></style>
