import {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react"
import {
  cacheExchange,
  Client as GqlClient,
  createClient,
  dedupExchange,
  errorExchange,
  makeOperation,
  subscriptionExchange,
} from "urql"
import { createClient as createWSClient } from "graphql-ws"
import { useSnackbar } from "notistack"
import { useSession } from "../hooks/session"
import { authExchange } from "@urql/exchange-auth"

class Client {
  private static readonly ApiPath = `/api`

  private readonly gqlClient: GqlClient
  private readonly onError?: (err: Error) => void

  constructor(props?: {
    session?: Nullable<string>
    onError?: (err: Error) => void
  }) {
    this.onError = props?.onError
    const wsClient = createWSClient({
      url: Client.wsUrl,
      connectionParams: props?.session
        ? { authorization: props.session }
        : undefined,
    })
    this.gqlClient = createClient({
      url: Client.ApiPath,
      exchanges: [
        dedupExchange,
        cacheExchange,
        errorExchange({
          onError: err => {
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

  private static get wsUrl() {
    return `${window.location.protocol.startsWith(`https`) ? `wss:` : `ws:`}//${
      window.location.host
    }${this.ApiPath}`
  }

  async getSelf() {
    const res = await this.gqlClient
      .query<{
        users: {
          edges: {
            node: User
          }[]
        }
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
    if (res.error) this.onError?.(res.error)
    return (
      res.data?.users.edges[0]?.node ??
      (() => {
        throw new Error(`current user not found`)
      })()
    )
  }

  async updateSelf(update: UserUpdate) {
    const res = await this.gqlClient
      .query<{ updateUsers: number }, UserUpdate>(
        /* GraphQL */ `
          mutation ($name: String, $email: String, $avatar: String) {
            updateUsers(update: { name: $name, email: $email, avatar: $avatar })
          }
        `,
        update
      )
      .toPromise()
    if (res.error) this.onError?.(res.error)
    if (!res.data?.updateUsers) throw new Error(`update self failed`)
  }

  async signup(input: SignUpInput) {
    const res = await this.gqlClient
      .query<{ signup: string }, SignUpInput>(
        /* GraphQL */ `
          mutation ($name: String!, $password: String!) {
            signup(input: { name: $name, password: $password })
          }
        `,
        input
      )
      .toPromise()
    if (res.error) this.onError?.(res.error)
    return res.data!.signup
  }

  async login(input: LogInInput) {
    const res = await this.gqlClient
      .query<{ login: string }, LogInInput>(
        /* GraphQL */ `
          mutation ($identity: String!, $password: String!) {
            login(input: { identity: $identity, password: $password })
          }
        `,
        input
      )
      .toPromise()
    if (res.error) this.onError?.(res.error)
    return res.data!.login
  }
}

const ClientContext = createContext<{
  client: Client
  reload: () => void
} | null>(null)

export function ClientProvider({ children }: PropsWithChildren) {
  const [session] = useSession()
  const { enqueueSnackbar } = useSnackbar()
  const onError = useCallback(
    (e: Error) => {
      enqueueSnackbar(e.message, { variant: `error` })
    },
    [enqueueSnackbar]
  )
  const createClient = useCallback(
    () => new Client({ onError, session }),
    [session, onError]
  )
  const [client, setClient] = useState(createClient())
  return (
    <ClientContext.Provider
      value={{
        client,
        reload: () => {
          setClient(createClient())
        },
      }}
    >
      {children}
    </ClientContext.Provider>
  )
}

export function useClient() {
  const client = useContext(ClientContext)?.client
  if (!client)
    throw new Error(
      `useClient must be called within a component wrapped by ClientContext`
    )
  return client
}

type AuthState = { token?: Nullable<string> }
