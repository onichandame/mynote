import {
  ExtractDocumentTypeFromTypedRxJsonSchema,
  RxCollection,
  RxDocument,
  toTypedRxJsonSchema,
} from "rxdb"

export const memoSchema = toTypedRxJsonSchema({
  type: `object`,
  version: 0,
  primaryKey: `id`,
  required: [`content`],
  properties: {
    id: { type: `string` },
    content: { type: `string` },
    weight: { type: `integer` },
    synced: { type: `boolean` },
    createdAt: { type: `string`, format: `date-time` },
    updatedAt: { type: `string`, format: `date-time` },
    deletedAt: { type: `string`, format: `date-time` },
  },
} as const)

export const memoMethods: MemoMethods = {
  softDelete: async function () {
    await this.atomicPatch({ deletedAt: new Date().toISOString() })
  },
  recover: async function () {
    await this.atomicPatch({ deletedAt: undefined })
  },
}

export const memoPreInsert = function (memo) {
  memo.id = window.crypto.randomUUID()
  memo.createdAt = new Date().toISOString()
  memo.synced = false
} as Parameters<MemoCollection["preInsert"]>[0]

export const memoPreSave = function (memo) {
  memo.updatedAt = new Date().toISOString()
  memo.synced = false
} as Parameters<MemoCollection["preSave"]>[0]

type MemoPlain = ExtractDocumentTypeFromTypedRxJsonSchema<typeof memoSchema>

type MemoMethods = {
  softDelete: (this: Memo) => Promise<void>
  recover: (this: Memo) => Promise<void>
}

export type Memo = RxDocument<MemoPlain, MemoMethods>

export type MemoCollection = RxCollection<MemoPlain, MemoMethods>
