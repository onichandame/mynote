export interface RequestOptions {
  /** whether the user should be notified about the progress of the request */
  notification?: { disablePending?: boolean; disableFinal?: boolean };
}
