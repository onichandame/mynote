import EventEmitter from "events";
import { Client, createClient } from "graphql-ws";
import { Connection, Note, NoteFilter, Sortings, User } from "../model";

import { Channel, ChannelOptions } from "./channel";
import { Event } from "./event";
import Schema from "./schema.graphql?raw";

/** interface to a backend service */
export class Service extends EventEmitter {
  private channels = new Set<Channel<any>>();
  private client: Client;
  constructor(url: string, session?: string) {
    super();
    this.client = createClient({ url, connectionParams: { session } });
  }

  public login(
    args: { nameOrEmail: string; password: string },
    opts?: ChannelOptions
  ) {
    const chan = this.request<string>(`login`, args, opts);
    return this.waitOnce(chan);
  }

  public signup(
    args: {
      name: string;
      password: string;
      email?: string;
      avatar?: string;
    },
    opts?: ChannelOptions
  ) {
    const chan = this.request<{ id: number }>(`signup`, args, opts);
    return this.waitOnce(chan);
  }

  public self(opts?: ChannelOptions) {
    const chan = this.request<User>(`self`, {}, opts);
    return this.waitOnce(chan);
  }

  public updateSelf(
    args: { name?: string; email?: string; avatar?: string },
    opts?: ChannelOptions
  ) {
    const chan = this.request<User>(`updateSelf`, args, opts);
    return this.waitOnce(chan);
  }

  public changePassword(
    oldPass: string,
    newPass: string,
    opts?: ChannelOptions
  ) {
    const chan = this.request<boolean>(
      `changePassword`,
      { oldPass, newPass },
      opts
    );
    return this.waitOnce(chan);
  }

  public createNote(
    args: { title: string; content: string },
    opts?: ChannelOptions
  ) {
    const chan = this.request<Note>(`createNote`, args, opts);
    return this.waitOnce(chan);
  }

  public listNotes(
    args?: {
      filter?: NoteFilter;
      sorting?: Sortings;
      first?: number;
      after?: string;
    },
    opts?: ChannelOptions
  ) {
    const chan = this.request<Connection<Note>>(`listNotes`, args, opts);
    return this.waitOnce(chan);
  }

  public getNote(id: number, opts?: ChannelOptions) {
    const chan = this.request<Note>(`getNote`, { id }, opts);
    return this.waitOnce(chan);
  }

  public updateNote(
    id: number,
    update: { title?: string; content?: string },
    opts?: ChannelOptions
  ) {
    const chan = this.request<Note>(`updateNote`, { id, ...update }, opts);
    return this.waitOnce(chan);
  }

  public deleteNote(id: number, opts?: ChannelOptions) {
    const chan = this.request<boolean>(`deleteNote`, { id }, opts);
    return this.waitOnce(chan);
  }

  public dispose() {
    this.channels.forEach((chan) => chan.emit(`close`, false));
    this.channels.clear();
  }

  on<TData = unknown>(
    ev: `data`,
    listener: (chan: Channel, data: TData) => void
  ): this;
  on(ev: `error`, listener: (chan: Channel, error: unknown) => void): this;
  on(ev: `close`, listener: (chan: Channel, success: boolean) => void): this;
  on(ev: `send`, listener: (chan: Channel) => void): this;
  on(ev: Event, listener: (chan: Channel, payload?: any) => void) {
    return super.on(ev, listener);
  }

  emit<TData = unknown>(ev: `data`, chan: Channel, data: TData): boolean;
  emit(ev: `error`, chan: Channel, error: unknown): boolean;
  emit(ev: `close`, chan: Channel, success: boolean): boolean;
  emit(ev: `send`, chan: Channel): boolean;
  emit(ev: Event, chan: Channel, payload?: any) {
    return super.emit(ev, chan, payload);
  }

  private request<
    TData = unknown,
    TVariable extends Record<string, unknown> = {}
  >(operationName: string, variables?: TVariable, opts?: ChannelOptions) {
    const chan = new Channel<TData>(operationName, opts);
    chan.on(`close`, (success) => this.emit(`close`, chan, success));
    chan.on(`error`, (e) => this.emit(`error`, chan, e));
    chan.on(`data`, (data) => this.emit(`data`, chan, data));
    const cleanup = this.client.subscribe<Record<typeof operationName, TData>>(
      { query: Schema, variables, operationName },
      {
        complete: () => chan.emit(`close`, true),
        error: (e) => {
          chan.emit(`error`, e);
          chan.emit(`close`, false);
        },
        next: (payload) =>
          payload.errors
            ? chan.emit(`error`, payload.errors)
            : payload.data
            ? chan.emit(`data`, payload.data[operationName])
            : chan.emit(`error`, new Error(`no data received`)),
      }
    );
    this.emit(`send`, chan);
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
