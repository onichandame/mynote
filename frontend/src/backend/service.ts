import EventEmitter from "events";
import { Client, createClient } from "graphql-ws";

import {
  Connection,
  CreateNoteInput,
  CreateUserInput,
  LoginInput,
  Note,
  NoteFilter,
  PasswordFilter,
  Pagination,
  Password,
  Sorting,
  SyncFromRemoteInput,
  UpdateNoteInput,
  UpdateUserInput,
  User,
  CreatePasswordInput,
  UpdatePasswordInput,
  CreatePeerInput,
  Peer,
  UpdatePeerInput,
  PeerFilter,
} from "../model";
import { Channel } from "./channel";
import { Event } from "./event";

/** interface to a backend service */
export class Service extends EventEmitter {
  private channels = new Set<Channel<any>>();
  private client: Client;
  constructor(url: string, session?: string) {
    super();
    this.client = createClient({ url, connectionParams: { session } });
  }

  public login(input: LoginInput) {
    const chan = this.subscribe<string>(
      `#graphql
      mutation ($input:LoginInput!) {
        login(input: $input)
      }`,
      {
        input,
      }
    );
    return this.waitOnce(chan);
  }

  public renewSession() {
    const chan = this.subscribe<string>(`#graphql
    mutation {
      renewSession
    }`);
    return this.waitOnce(chan);
  }

  public checkPassword(password: string) {
    const chan = this.subscribe<boolean>(
      `#graphql
      mutation ($password:String!){
        validatePassword(password:$password)
      }`,
      { password }
    );
    return this.waitOnce(chan);
  }

  public createUser(input: CreateUserInput) {
    const chan = this.subscribe<User>(
      `#graphql
      mutation ($input:UserInput!){
        createUser(input: $input){
          ${User.fields.join(` `)}
        }
      }`,
      { input }
    );
    return this.waitOnce(chan);
  }

  public listUsers() {
    const chan = this.subscribe<Connection<User>>(
      `#graphql
      query{
        listUsers{
          edges{
            node{
              ${User.fields.join(` `)}
            }
          }
        }
      }`
    );
    return this.waitOnce(chan);
  }

  public updateUsers(update: UpdateUserInput) {
    const chan = this.subscribe<number>(
      `#graphql
      mutation($update:UserUpdate!){
        updateUsers(update:$update)
      }`,
      { update }
    );
    return this.waitOnce(chan);
  }

  public createNote(input: CreateNoteInput) {
    const chan = this.subscribe<Note>(
      `#graphql
      mutation($input:NoteInput!){
        createNote(input:$input){
          ${Note.fields.join(` `)}
        }
      }`,
      { input }
    );
    return this.waitOnce(chan);
  }

  public listNotes(
    filter?: NoteFilter,
    paging?: Pagination,
    sorting?: Sorting<Note>[]
  ) {
    const chan = this.subscribe<Connection<Note>>(
      `#graphql
      query($filter:NoteFilter,$paging:Pagination,$sorting:[NoteSort]) {
        listNotes(filter:$filter,paging:$paging,sorting:$sorting){
          edges{
            node{
              ${Note.fields.join(` `)}
            }
          }
        }
      }`,
      {
        filter,
        sorting,
        paging,
      }
    );
    return this.waitOnce(chan);
  }

  public updateNotes(update: UpdateNoteInput, filter?: NoteFilter) {
    const chan = this.subscribe<number>(
      `#graphql
    mutation($filter:NoteFilter,$update:NoteUpdate!){
      updateNotes(filter:$filter,update:$update)
    }`,
      { filter, update }
    );
    return this.waitOnce(chan);
  }

  public deleteNotes(filter: NoteFilter) {
    const chan = this.subscribe<boolean>(
      `#graphql
    mutation($filter:NoteFilter){
      deleteNotes(filter:$filter)
    }`,
      { filter }
    );
    return this.waitOnce(chan);
  }

  public listPasswords(
    filter?: PasswordFilter,
    paging?: Pagination,
    sorting?: Sorting<Password>[]
  ) {
    const chan = this.subscribe<Connection<Password>>(
      `#graphql
      query($filter:PasswordFilter,$paging:Pagination,$sorting:[PasswordSort]) {
        listPasswords(filter:$filter,paging:$paging,sorting:$sorting){
          edges{
            node{
              ${Password.fields.join(` `)}
            }
          }
        }
      }`,
      { filter, paging, sorting }
    );
    return this.waitOnce(chan);
  }

  public createPassword(input: CreatePasswordInput) {
    const chan = this.subscribe<Password>(
      `#graphql
    mutation($input:PasswordInput!){
      createPassword(input:$input){
        ${Password.fields.join(` `)}
      }
    }
    `,
      { input }
    );
    return this.waitOnce(chan);
  }

  public updatePasswords(update: UpdatePasswordInput, filter?: PasswordFilter) {
    const chan = this.subscribe<number>(
      `#graphql
    mutation($filter:PasswordFilter,$update:PasswordUpdate!){
      updatePasswords(filter:$filter,update:$update)
    }`,
      { filter, update }
    );
    return this.waitOnce(chan);
  }

  public createPeer(input: CreatePeerInput) {
    const chan = this.subscribe<Peer>(
      `#graphql
      mutation ($input:PeerInput!){
        createPeer(input: $input){
          ${Peer.fields.join(` `)}
        }
      }`,
      { input }
    );
    return this.waitOnce(chan);
  }

  public updatePeers(update: UpdatePeerInput, filter?: PeerFilter) {
    const chan = this.subscribe<number>(
      `#graphql
    mutation($filter:PeerFilter,$update:PeerUpdate!){
      updatePeers(filter:$filter,update:$update)
    }`,
      { filter, update }
    );
    return this.waitOnce(chan);
  }

  public listPeers(
    filter?: PeerFilter,
    paging?: Pagination,
    sorting?: Sorting<Peer>[]
  ) {
    const chan = this.subscribe<Connection<Peer>>(
      `#graphql
      query($filter:PeerFilter,$paging:Pagination,$sorting:[PeerSort]) {
        listPeers(filter:$filter,paging:$paging,sorting:$sorting){
          edges{
            node{
              ${Peer.fields.join(` `)}
            }
          }
        }
      }`,
      { filter, paging, sorting }
    );
    return this.waitOnce(chan);
  }

  public syncFromRemote(input: SyncFromRemoteInput) {
    const chan = this.subscribe<boolean>(
      `#graphql
    mutation($input:SyncFromRemoteInput!) {
      syncFromRemote(input:$input)
    }`,
      { input }
    );
    return this.waitOnce(chan);
  }

  public syncFromPeer(peerId: number) {
    const chan = this.subscribe<boolean>(
      `#graphql
    mutation($peerId:Int!){
      syncFromPeer(peerId:$peerId)
    }`,
      { peerId }
    );
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
  on(ev: Event, listener: (chan: Channel, payload?: any) => void) {
    return super.on(ev, listener);
  }

  emit<TData = unknown>(ev: `data`, chan: Channel, data: TData): boolean;
  emit(ev: `error`, chan: Channel, error: unknown): boolean;
  emit(ev: `close`, chan: Channel, success: boolean): boolean;
  emit(ev: Event, chan: Channel, payload?: any) {
    return super.emit(ev, chan, payload);
  }

  private subscribe<
    TData = unknown,
    TVariable extends Record<string, unknown> = {}
  >(query: string, variables?: TVariable) {
    const chan = new Channel<TData>();
    chan.on(`close`, (success) => this.emit(`close`, chan, success));
    chan.on(`error`, (e) => {
      this.emit(`error`, chan, e);
      chan.emit(`close`, false);
    });
    chan.on(`data`, (data) => this.emit(`data`, chan, data));
    const cleanup = this.client.subscribe<Record<string, TData>>(
      { query, variables },
      {
        complete: () => chan.emit(`close`, true),
        error: (e) => chan.emit(`error`, e),
        next: (payload) => {
          if (payload.errors) chan.emit(`error`, payload.errors);
          else if (payload.data) {
            const responses = Object.values(payload.data);
            if (responses[0]) chan.emit(`data`, responses[0]);
            else chan.emit(`error`, new Error(`no data received`));
          } else chan.emit(`error`, new Error(`no data received`));
        },
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
