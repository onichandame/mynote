import { useClient } from "./client";

type PlainObject<TValue = unknown> = Record<string, TValue>;

// Query/Mutation
export const useFetcher = <
  TData extends PlainObject = {},
  TVariable extends PlainObject = {}
>(
  query: string
) => {
  const client = useClient();
  return (variables: TVariable) => {
    if (!client) throw new Error(`client not initialized`);
    let result: TData;
    let resolve = (_: TData) => {};
    let reject = (_: unknown) => {};
    const promise = new Promise<TData>((r, j) => {
      resolve = r;
      reject = j;
    });
    const stop = client.subscribe<TData>(
      { query, variables },
      {
        complete: () => resolve(result),
        error: (e) => reject(e),
        next: (payload) => {
          if (payload.errors) reject(payload.errors);
          else if (payload.data) result = payload.data;
        },
      }
    );
    return [promise, stop] as const;
  };
};
export const useSubscriber = <
  TData extends PlainObject = {},
  TVariable extends PlainObject = {}
>(
  query: string
) => {
  const client = useClient();
  return (
    variables: TVariable,
    handlers: { onData: (_: TData) => void; onError?: (_: unknown) => void }
  ) => {
    if (!client) throw new Error(`client not initialized`);
    let resolve = () => {};
    let reject = (_: unknown) => {};
    const promise = new Promise<void>((r, j) => {
      resolve = r;
      reject = j;
    });
    const stop = client.subscribe<TData>(
      { query, variables },
      {
        complete: resolve,
        error: reject,
        next: (payload) => {
          if (payload.errors)
            handlers.onError && handlers.onError(payload.errors);
          else if (payload.data) handlers.onData(payload.data);
        },
      }
    );
    return [promise, stop];
  };
};
