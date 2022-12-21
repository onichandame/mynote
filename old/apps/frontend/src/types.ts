type Connection<T> = { edges: { node: T }[] }
type User = { id: number; name: string; email?: string }
type UpdateUserInput = Pick<Partial<User>, "name" | "email">
type SignUpInput = { name: string; password: string }
type LogInInput = { identity: string; password: string }
type ChangePasswordInput = { password: string }
type Memo = {
  id: number
  content: string
  weight?: number
  createdAt: Date
  updatedAt?: Date
}
type CreateMemoInput = Pick<Memo, "content">
type UpdateMemoInput = Pick<Partial<Memo>, "content" | "weight">

type Nullable<T> = T | null | undefined
