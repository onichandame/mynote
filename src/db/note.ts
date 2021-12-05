import { Base } from './base'

export interface Note extends Base {
  title: string
  content: string
  createdAt: Date
  updatedAt?: Date
}
