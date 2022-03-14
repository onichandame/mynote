import { Bucket } from "@olian/typescript-helpers";
import EventEmitter from "events";
import { Client, createClient } from "graphql-ws";
import { User } from "../model";

import { Channel } from "./channel";
import { Event } from "./event";
import Schema from "./schema.graphql?raw";

/** interface to a backend service */
export class Backend extends EventEmitter {
  private channels = new Set<Channel<any>>();
  private client: Client;
  constructor(url: string, session?: string) {
    super();
    this.client = createClient({ url, connectionParams: { session } });
  }

  public login(nameOrEmail: string, password: string) {
    const chan = this.request<string>(`login`, { nameOrEmail, password });
    return this.waitOnce(chan);
  }

  public signup(args: {
    name: string;
    password: string;
    email?: string;
    avatar?: string;
  }) {
    const chan = this.request<{ id: number }>(`signup`, args);
    return this.waitOnce(chan);
  }

  public self() {
    const chan = this.request<User>(`self`);
    return this.waitOnce(chan);
  }

  public dispose() {
    this.channels.forEach((chan) => chan.emit(`close`));
    this.channels.clear();
  }

  on<TData = unknown>(
    ev: `data`,
    listener: (chanId: string, data: TData) => void
  ): this;
  on(ev: `error`, listener: (chanId: string, error: unknown) => void): this;
  on(ev: `close`, listener: (chanId: string) => void): this;
  on(ev: Event, listener: (chanId: string, payload?: any) => void) {
    return super.on(ev, listener);
  }

  emit<TData = unknown>(ev: `data`, chanId: number, data: TData): boolean;
  emit(ev: `error`, chanId: number, error: unknown): boolean;
  emit(ev: `close`, chanId: number): boolean;
  emit(ev: Event, chanId: number, payload?: any) {
    return super.emit(ev, chanId, payload);
  }

  private request<
    TData = unknown,
    TVariable extends Record<string, unknown> = {}
  >(operationName: string, variables?: TVariable) {
    const chan = new Channel<TData>();
    chan.on(`close`, () => this.emit(`close`, chan.id));
    chan.on(`error`, (e) => this.emit(`error`, chan.id, e));
    chan.on(`data`, (data) => this.emit(`data`, chan.id, data));
    const cleanup = this.client.subscribe<Record<typeof operationName, TData>>(
      { query: Schema, variables, operationName },
      {
        complete: () => chan.emit(`close`),
        error: (e) => {
          chan.emit(`error`, e);
          chan.emit(`close`);
        },
        next: (payload) =>
          payload.errors
            ? chan.emit(`error`, payload.errors)
            : payload.data
            ? chan.emit(`data`, payload.data[operationName])
            : chan.emit(`error`, new Error(`no data received`)),
      }
    );
    chan.on(`close`, cleanup);
    this.channels.add(chan);
    chan.on(`close`, () => this.channels.delete(chan));
    return chan;
  }

  private waitOnce<TData = unknown>(chan: Channel<TData>) {
    return new Promise<TData>(async (r, j) => {
      chan.on(`data`, (data) => r(data));
      chan.on(`error`, (e) => j(e));
      chan.on(`close`, () => j(new Error(`channel closed`)));
    });
  }
}
