import EventEmitter from "events";

import { Event } from "./event";

export class Channel<TData = unknown> extends EventEmitter {
  private static lastChan = 0;

  public id: number;

  private closed = false;

  /**
   * @param request - name of the requested function
   * @param opts - options of the channel(request)
   */
  constructor(public request: string, public opts?: ChannelOptions) {
    super();
    this.id = ++Channel.lastChan;
  }

  on(ev: `data`, listener: (data: TData) => void): this;
  on(ev: `error`, listener: (data: unknown) => void): this;
  on(ev: `close`, listener: (success: boolean) => void): this;
  on(ev: Event, listener: (payload?: any) => void) {
    return super.on(ev, listener);
  }
  emit(ev: `data`, data: TData): boolean;
  emit(ev: `error`, data: unknown): boolean;
  emit(ev: `close`, success: boolean): boolean;
  emit(ev: Event, payload?: any) {
    if (!this.closed) {
      const res = super.emit(ev, payload);
      if (ev === `close`) {
        this.closed = true;
        this.removeAllListeners();
      }
      return res;
    } else return false;
  }
}

export interface ChannelOptions {
  /** whether the user should be notified about the progress */
  notification?: { disablePending?: boolean; disableFinal?: boolean } | boolean;
}
