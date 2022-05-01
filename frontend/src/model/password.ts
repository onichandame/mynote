import { Owned } from "./base";

export class Password extends Owned {
  group_id?: number;
  title!: string;
  password!: string;
  url?: string;

  static get fields(): string[] {
    return super.fields.concat([
      `group_id`,
      `title`,
      `password`,
      `url`,
    ] as (keyof Password)[]);
  }
}

export class PasswordGroup extends Owned {
  parent_id?: number;
  title!: string;

  static get fields(): string[] {
    return super.fields.concat([
      `parent_id`,
      `title`,
    ] as (keyof PasswordGroup)[]);
  }
}
