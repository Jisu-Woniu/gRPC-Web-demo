import "./assets/main.css";

import { createApp } from "vue";
import App from "./App.vue";
import { VueQueryPlugin } from "@tanstack/vue-query";
import { createGrpcWebTransport } from "@connectrpc/connect-web";
import { defaultTransport } from "@/useTransport";

const transport = createGrpcWebTransport({
  baseUrl: "http://127.0.0.1:50051",
});

createApp(App)
  .use(VueQueryPlugin)
  .provide(defaultTransport, transport)
  .mount("#app");
