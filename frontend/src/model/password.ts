import { IsBoolean, IsOptional, IsString, IsUrl } from "class-validator";
import { DateFilter, IntFilter, StringFilter } from ".";
import { UsernameValidator, ValidateIfNotEmpty } from "../common";
import { Owned } from "./base";

export class Password extends Owned {
  isLocal!: boolean;
  groupId?: number;
  title!: string;
  icon?: string;
  password!: string;
  url?: string;
  username?: string;

  static get fields(): string[] {
    return super.fields.concat([
      `isLocal`,
      `groupId`,
      `title`,
      `icon`,
      `password`,
      `url`,
      `username`,
    ] as (keyof Password)[]);
  }
}

export class PasswordGroup extends Owned {
  parent_id?: number;
  title!: string;

  static get fields(): string[] {
    return super.fields.concat([
      `parentId`,
      `title`,
    ] as (keyof PasswordGroup)[]);
  }
}

export class PasswordFilter {
  id?: InstanceType<typeof IntFilter>;
  uuid?: InstanceType<typeof StringFilter>;
  deletedAt?: InstanceType<typeof DateFilter>;
}

export class CreatePasswordInput {
  @IsBoolean()
  isLocal!: boolean;
  @IsString()
  title!: string;
  @ValidateIfNotEmpty()
  @IsString()
  icon?: string;
  @IsString()
  password!: string;
  @ValidateIfNotEmpty()
  @IsUrl({ protocols: [`ws`, `wss`, `http`, `https`, `ftp`] })
  url?: string;
  @ValidateIfNotEmpty()
  @UsernameValidator()
  username?: string;
}

export class UpdatePasswordInput {
  @ValidateIfNotEmpty()
  @IsBoolean()
  isLocal?: boolean;
  @ValidateIfNotEmpty()
  @IsString()
  title?: string;
  @IsOptional()
  @IsString()
  icon?: string | null;
  @ValidateIfNotEmpty()
  @IsString()
  password?: string;
  @ValidateIfNotEmpty()
  @IsUrl({ protocols: [`ws`, `wss`, `http`, `https`, `ftp`] })
  url?: string;
  @ValidateIfNotEmpty()
  @UsernameValidator()
  username?: string;
  @IsOptional()
  @IsString()
  deletedAt?: Date;
}
