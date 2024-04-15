import { inject, type InjectionKey } from "vue";
import { ConnectError, type Transport } from "@connectrpc/connect";

const fallbackTransportError = new ConnectError(
  "To use Connect, you must provide a `Transport`: a simple object that handles `unary` and `stream` requests. `Transport` objects can easily be created by using `@connectrpc/connect-web`'s exports `createConnectTransport` and `createGrpcWebTransport`. see: https://connectrpc.com/docs/web/getting-started for more info.",
);
export const fallbackTransport = {
  unary: () => {
    throw fallbackTransportError;
  },
  stream: () => {
    throw fallbackTransportError;
  },
};

export const defaultTransport: InjectionKey<Transport> = Symbol();

export const useTransport = () => inject(defaultTransport, fallbackTransport);
