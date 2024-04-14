import { useTransport } from "./useTransport";
import {
  type ConnectQueryKey,
  type MethodUnaryDescriptor,
} from "@connectrpc/connect-query";
import { Message, type PartialMessage } from "@bufbuild/protobuf";
import {
  useMutation as tsUseMutation,
  type UseMutationOptions as TSUseMutationOptions,
} from "@tanstack/vue-query";
import type { CallOptions, ConnectError, Transport } from "@connectrpc/connect";

export type UseMutationOptions<
  I extends Message<I>,
  O extends Message<O>,
> = Omit<
  TSUseMutationOptions<O, ConnectError, PartialMessage<I>, ConnectQueryKey<I>>,
  "mutationFn"
> & {
  transport?: Transport;
  callOptions?: CallOptions;
};

export const useMutation = <I extends Message<I>, O extends Message<O>>(
  methodSig: MethodUnaryDescriptor<I, O>,
  { transport, callOptions, ...queryOptions }: UseMutationOptions<I, O> = {},
) => {
  const transportToUse = transport ?? useTransport();

  const mutationFn = async (input: PartialMessage<I>) => {
    const result = await transportToUse.unary(
      {
        typeName: methodSig.service.typeName,
        methods: {},
      },
      methodSig,
      callOptions?.signal,
      callOptions?.timeoutMs,
      callOptions?.headers,
      input,
    );
    return result.message;
  };
  return tsUseMutation({ ...queryOptions, mutationFn });
};
