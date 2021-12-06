import Dexie, { Table } from 'dexie'

import { Note } from './note'

class DB extends Dexie {
  notes!: Table<Note>
  constructor() {
    super(`mynotes_root`)
    this.version(1).stores({
      notes: `++id,createdAt`,
    })
  }
}

export const db = new DB()
