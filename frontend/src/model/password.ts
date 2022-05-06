import { IsEmail, IsOptional, IsString, IsUrl } from "class-validator";
import { DateFilter, IntFilter, StringFilter } from ".";
import { UsernameValidator, ValidateIfNotEmpty } from "../common";
import { Owned } from "./base";

export class Password extends Owned {
  group_id?: number;
  title!: string;
  password!: string;
  url?: string;
  username?: string;
  email?: string;

  static get fields(): string[] {
    return super.fields.concat([
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
  @IsString()
  title!: string;
  @IsString()
  password!: string;
  @ValidateIfNotEmpty()
  @IsUrl()
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
