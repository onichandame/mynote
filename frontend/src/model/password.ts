import {
  IsBoolean,
  IsEmail,
  IsOptional,
  IsString,
  IsUrl,
} from "class-validator";
import { DateFilter, IntFilter, StringFilter } from ".";
import { UsernameValidator, ValidateIfNotEmpty } from "../common";
import { Owned } from "./base";

export class Password extends Owned {
  isLocal!: boolean;
  groupId?: number;
  title!: string;
  password!: string;
  url?: string;
  username?: string;
  email?: string;

  static get fields(): string[] {
    return super.fields.concat([
      `isLocal`,
      `groupId`,
      `title`,
      `password`,
      `url`,
      `username`,
      `email`,
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
  @IsString()
  password!: string;
  @ValidateIfNotEmpty()
  @IsUrl({ protocols: [`ws`, `wss`, `http`, `https`, `ftp`] })
  url?: string;
  @ValidateIfNotEmpty()
  @UsernameValidator()
  username?: string;
  @ValidateIfNotEmpty()
  @IsEmail()
  email?: string;
}

export class UpdatePasswordInput {
  @ValidateIfNotEmpty()
  @IsBoolean()
  isLocal?: boolean;
  @ValidateIfNotEmpty()
  @IsString()
  title?: string;
  @ValidateIfNotEmpty()
  @IsString()
  password?: string;
  @ValidateIfNotEmpty()
  @IsUrl({ protocols: [`ws`, `wss`, `http`, `https`, `ftp`] })
  url?: string;
  @ValidateIfNotEmpty()
  @UsernameValidator()
  username?: string;
  @ValidateIfNotEmpty()
  @IsEmail()
  email?: string;
  @IsOptional()
  @IsString()
  deletedAt?: Date;
}
