import { IsUrl } from "class-validator";

import { LoginInput } from ".";

export class SyncFromRemoteInput extends LoginInput {
  @IsUrl()
  url!: string;
}
