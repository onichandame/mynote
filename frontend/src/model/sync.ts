import { IsUrl } from "class-validator";

import { LoginInput } from ".";

export class SyncFromRemoteInput extends LoginInput {
  @IsUrl({ protocols: [`ws`, `wss`] })
  url!: string;
}
