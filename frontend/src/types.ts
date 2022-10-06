type Connection<T> = { edges: { node: T }[] }
type User = { id: number; name: string; email?: string; avatar?: string }
type UpdateUserInput = Pick<Partial<User>, "name" | "email" | "avatar">
type SignUpInput = { name: string; password: string }
type LogInInput = { identity: string; password: string }
type ChangePasswordInput = { password: string }
type Memo = {
  id: number
  content: string
  createdAt: Date
  updatedAt?: Date
}
type CreateMemoInput = Pick<Memo, "content">
type UpdateMemoInput = Pick<Partial<Memo>, "content">

type Nullable<T> = T | null | undefined
