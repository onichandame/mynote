import {
  createContext,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react"
import {
  Client as GqlClient,
  createClient,
  dedupExchange,
  errorExchange,
  makeOperation,
  subscriptionExchange,
} from "urql"
import { createClient as createWSClient } from "graphql-ws"
import { useSnackbar } from "notistack"
import { authExchange } from "@urql/exchange-auth"
import Ws from "isomorphic-ws"

import { useSession } from "./session"

class Client {
  private readonly session?: Nullable<string>
  private readonly gqlClient: GqlClient
  private readonly onError?: (err: Error) => void

  constructor(
    url: string,
    props?: {
      session?: Nullable<string>
      onError?: (err: Error) => void
    }
  ) {
    this.session = props?.session
    this.onError = props?.onError
    const wsClient = createWSClient({
      webSocketImpl: Ws,
      url: getWsUrl(url),
      connectionParams: props?.session
        ? { authorization: props.session }
        : undefined,
    })
    this.gqlClient = createClient({
      url,
      exchanges: [
        dedupExchange,
        errorExchange({
          onError: err => {
            console.warn(err)
            props?.onError?.(err)
          },
        }),
        authExchange<AuthState>({
          getAuth: async () => ({ token: props?.session }),
          addAuthToOperation: ({ authState, operation }) => {
            if (!authState?.token) return operation
            const fetchOptions =
              typeof operation.context.fetchOptions === "function"
                ? operation.context.fetchOptions()
                : operation.context.fetchOptions || {}
            return makeOperation(operation.kind, operation, {
              ...operation.context,
              fetchOptions: {
                ...fetchOptions,
                headers: {
                  ...fetchOptions.headers,
                  Authorization: `Bearer ${authState.token}`,
                },
              },
            })
          },
        }),
        subscriptionExchange({
          forwardSubscription: operation => ({
            subscribe: sink => ({
              unsubscribe: wsClient.subscribe(operation, sink),
            }),
          }),
          enableAllOperations: true,
        }),
      ],
    })
  }

  async getSelf() {
    if (!this.session) return
    const res = await this.gqlClient
      .query<{
        users: Connection<User>
      }>(
        /* GraphQL */ `
          query {
            users {
              edges {
                node {
                  id
                  name
                  email
                  avatar
                }
              }
            }
          }
        `,
        {}
      )
      .toPromise()
    if (res.data?.users.edges[0]?.node) return res.data?.users.edges[0]?.node
  }

  async updateSelf(update: UpdateUserInput) {
    if (!this.session) return
    const res = await this.gqlClient
      .mutation<{ updateUsers: number }, UpdateUserInput>(
        /* GraphQL */ `
          mutation ($name: String, $email: String, $avatar: String) {
            updateUsers(update: { name: $name, email: $email, avatar: $avatar })
          }
        `,
        update
      )
      .toPromise()
    if (!res.data?.updateUsers) throw new Error(`update self failed`)
  }

  async signup(input: SignUpInput) {
    const res = await this.gqlClient
      .mutation<{ signup: string }, SignUpInput>(
        /* GraphQL */ `
          mutation ($name: String!, $password: String!) {
            signup(input: { name: $name, password: $password })
          }
        `,
        input
      )
      .toPromise()
    if (res.data?.signup) return res.data!.signup
  }

  async login(input: LogInInput) {
    const res = await this.gqlClient
      .mutation<{ login: string }, LogInInput>(
        /* GraphQL */ `
          mutation ($identity: String!, $password: String!) {
            login(input: { identity: $identity, password: $password })
          }
        `,
        input
      )
      .toPromise()
    if (res.data?.login) return res.data.login
  }

  async changePassword(input: ChangePasswordInput) {
    const res = await this.gqlClient
      .mutation<{ changePassword: boolean }, ChangePasswordInput>(
        /* GraphQL */ `
          mutation ($password: String!) {
            changePassword(input: { password: $password })
          }
        `,
        input
      )
      .toPromise()
    if (!res.data?.changePassword) throw new Error(`change password failed`)
  }

  async createMemo(input: CreateMemoInput) {
    const res = await this.gqlClient
      .mutation<{ createMemo: Memo }, CreateMemoInput>(
        /* GraphQL */ `
          mutation ($content: String!) {
            createMemo(input: { content: $content }) {
              id
              content
              createdAt
              updatedAt
            }
          }
        `,
        input
      )
      .toPromise()
    return res.data?.createMemo
  }

  async listMemos() {
    const res = await this.gqlClient
      .query<{ memos: Connection<Memo> }>(
        /* GraphQL */ `
          query {
            memos(
              sorting: [
                { field: WEIGHT, direction: ASC }
                { field: CREATED_AT, direction: DESC }
              ]
            ) {
              edges {
                node {
                  id
                  content
                  createdAt
                  updatedAt
                }
              }
            }
          }
        `,
        {}
      )
      .toPromise()
    return res.data?.memos
  }

  async getMemo(id: number) {
    const res = await this.gqlClient
      .query<{ memos: Connection<Memo> }, { id: number }>(
        /* GraphQL */ `
          query ($id: Int!) {
            memos(filter: { id: { eq: $id } }, paging: { first: 1 }) {
              edges {
                node {
                  id
                  content
                  createdAt
                  updatedAt
                }
              }
            }
          }
        `,
        { id }
      )
      .toPromise()
    return res.data?.memos?.edges?.[0]?.node
  }

  async updateMemo(id: number, update: UpdateMemoInput) {
    const res = await this.gqlClient
      .mutation<
        { updateMemos: number },
        { id: number; update: UpdateMemoInput }
      >(
        /* GraphQL */ `
          mutation ($id: Int!, $update: MemoUpdate!) {
            updateMemos(filter: { id: { eq: $id } }, update: $update)
          }
        `,
        { id, update }
      )
      .toPromise()
    if (!res.data?.updateMemos) this.onError?.(new Error(`update memo failed`))
  }

  async deleteMemo(id: number) {
    const res = await this.gqlClient
      .mutation<{ deleteMemos: number }, { id: number }>(
        /* GraphQL */ `
          mutation ($id: Int!) {
            deleteMemos(filter: { id: { eq: $id } })
          }
        `,
        {
          id,
        }
      )
      .toPromise()
    if (!res.data?.deleteMemos) this.onError?.(new Error(`delete memo failed`))
  }

  private async uploadParams() {
    const res = await this.gqlClient
      .query<{ uploadParams: string }>(
        /* GraphQL */ `
          query {
            uploadParams
          }
        `,
        {}
      )
      .toPromise()
    return res.data?.uploadParams
  }

  async uploadFile(file: File) {
    const data = new FormData()
    data.append(`file`, file)
    const paramStr = await this.uploadParams()
    if (!paramStr) throw new Error(`cannot upload file`)
    for (const [key, val] of Object.entries(JSON.parse(paramStr))) {
      data.append(key, val!.toString())
    }
    const res = await fetch(process.env.GATSBY_CDN_UPLOAD_URL!, {
      method: `POST`,
      body: data,
    }).then(async v => {
      if (v.status >= 400) throw new Error(await v.text())
      return v.json()
    })
    return res.secure_url
  }
}

const ClientContext = createContext<Client | null>(null)

export function ClientProvider({ children }: PropsWithChildren) {
  const [session] = useSession()
  const [client, setClient] = useState<Client | null>(null)
  const { enqueueSnackbar } = useSnackbar()
  const onError = (e: Error) => {
    enqueueSnackbar(e.message, { variant: `error` })
  }
  useEffect(() => {
    let active = true
    ;(async () => {
      try {
        if (active) {
          setClient(
            new Client([process.env.GATSBY_API_HOST, `api`].join(`/`), {
              onError,
              session,
            })
          )
        }
      } catch (e) {
        enqueueSnackbar(e instanceof Error ? e.message : JSON.stringify(e), {
          variant: `error`,
          persist: true,
        })
      }
    })()
    return () => {
      active = false
    }
  }, [session])
  return (
    <ClientContext.Provider value={client}>{children}</ClientContext.Provider>
  )
}

export function useClient() {
  const client = useContext(ClientContext)
  return client
}

function getWsUrl(url: string) {
  return url.replace(/^http/, `ws`)
}

type AuthState = { token?: Nullable<string> }
