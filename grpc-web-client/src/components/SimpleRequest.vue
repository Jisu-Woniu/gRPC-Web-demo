<script setup lang="ts">
import { EchoService } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_es/echo/v1/echo_connect";
import { GreetService } from "@buf/jisu-woniu_grpc-web-demo.connectrpc_es/greet/v1/greet_connect";
import { createPromiseClient } from "@connectrpc/connect";
import { useMutation } from "@tanstack/vue-query";
import { ref } from "vue";
import { useTransport } from "@/useTransport";

const input = ref<string>();
const transport = useTransport();
const echoClient = createPromiseClient(EchoService, transport);
const greetClient = createPromiseClient(GreetService, transport);

const echoMutation = useMutation({ mutationFn: echoClient.echo });
const greetMutation = useMutation({ mutationFn: greetClient.greet });

const name = echoMutation.data;
const greeting = greetMutation.data;

const onclick = async () => {
  await Promise.all([
    echoMutation.mutateAsync({ message: input.value }),
    greetMutation.mutateAsync({ name: input.value }),
  ]);
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
