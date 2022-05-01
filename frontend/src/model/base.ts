export class Base {
  id!: number;
  createdAt!: Date;
  updatedAt?: Date;
  deletedAt?: Date;

  static get fields(): string[] {
    return [`id`, `createdAt`, `updatedAt`, `deletedAt`] as (keyof Base)[];
  }
}

export class Universal extends Base {
  uuid!: string;
  lamportClock?: number;
  static get fields(): string[] {
    return super.fields.concat([`uuid`, `lamportClock`] as (keyof Universal)[]);
  }
}

export class Owned extends Universal {
  userId!: number;

  static get fields(): string[] {
    return super.fields.concat([`userId`] as (keyof Owned)[]);
  }
}
