import { IsOptional, IsString } from "class-validator";
import { DateFilter, StringFilter } from ".";

import { Owned } from "./base";
import { IntFilter } from "./filter";

export class Note extends Owned {
  title!: string;
  content!: string;

  static get fields(): string[] {
    return super.fields.concat([`title`, `content`] as (keyof Note)[]);
  }
}

export class NoteFilter {
  id?: InstanceType<typeof IntFilter>;
  uuid?: InstanceType<typeof StringFilter>;
  deletedAt?: InstanceType<typeof DateFilter>;
}

export class CreateNoteInput {
  @IsString()
  title!: string;
  @IsString()
  content!: string;
}

export class UpdateNoteInput {
  @IsOptional()
  @IsString()
  title?: string;
  @IsOptional()
  @IsString()
  content?: string;
  @IsOptional()
  @IsString()
  deletedAt?: Date;
}
