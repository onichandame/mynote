type User = { id: number; name: string; email?: string; avatar?: string }
type UserUpdate = Pick<Partial<User>, "name" | "email" | "avatar">
type SignUpInput = { name: string; password: string }
type LogInInput = { identity: string; password: string }
type ChangePasswordInput = { password: string }

type Nullable<T> = T | null | undefined
