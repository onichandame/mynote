import {
  ExtractDocumentTypeFromTypedRxJsonSchema,
  RxCollection,
  toTypedRxJsonSchema,
} from "rxdb"

export const memoSchema = toTypedRxJsonSchema({
  type: `object`,
  version: 0,
  primaryKey: `id`,
  required: [`id`, `content`],
  properties: {
    id: { type: `integer` },
    content: { type: `string` },
    weight: { type: `integer` },
  },
} as const)

type Memo = ExtractDocumentTypeFromTypedRxJsonSchema<typeof memoSchema>

export type MemoCollection = RxCollection<Memo>
