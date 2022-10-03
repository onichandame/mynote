import {
  createContext,
  PropsWithChildren,
  useContext,
  useEffect,
  useRef,
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
import { authExchange } from "@urql/exchange-auth"
import { useSession } from "./session"

class Client {
  private static readonly ApiPath = `/api`
  private static ApiHost = process.env.GATSBY_API_HOST

  private readonly session?: Nullable<string>
  private readonly gqlClient: GqlClient
  private readonly onError?: (err: Error) => void

  constructor(props?: {
    session?: Nullable<string>
    onError?: (err: Error) => void
  }) {
    this.session = props?.session
    this.onError = props?.onError
    const wsClient = createWSClient({
      url: Client.wsUrl,
      connectionParams: props?.session
        ? { authorization: props.session }
        : undefined,
    })
    this.gqlClient = createClient({
      url: Client.httpUrl,
      exchanges: [
        dedupExchange,
        cacheExchange,
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

  private static get httpUrl() {
    if (this.ApiHost) return `${this.ApiHost}${this.ApiPath}`
    else return this.ApiPath
  }

  private static get wsUrl() {
    const host = this.ApiHost?.startsWith(`http`)
      ? this.ApiHost
      : `${window.location.protocol}//${window.location.host}:${window.location.port}`
    return `${host.replace(/^http/, "ws")}${this.ApiPath}`
  }

  async getSelf() {
    if (!this.session) return
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
    else return res.data?.users.edges[0]?.node
  }

  async updateSelf(update: UserUpdate) {
    if (!this.session) return
    const res = await this.gqlClient
      .mutation<{ updateUsers: number }, UserUpdate>(
        /* GraphQL */ `
          mutation ($name: String, $email: String, $avatar: String) {
            updateUsers(update: { name: $name, email: $email, avatar: $avatar })
          }
        `,
        update
      )
      .toPromise()
    if (res.error) this.onError?.(res.error)
    else if (!res.data?.updateUsers) throw new Error(`update self failed`)
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
    if (res.error) this.onError?.(res.error)
    else return res.data!.signup
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
    if (res.error) this.onError?.(res.error)
    else return res.data!.login
  }
}

const ClientContext = createContext<Client | null>(null)

export function ClientProvider({ children }: PropsWithChildren) {
  const [session] = useSession()
  const { enqueueSnackbar } = useSnackbar()
  const onError = (e: Error) => {
    enqueueSnackbar(e.message, { variant: `error` })
  }
  const [client, setClient] = useState(new Client({ onError, session }))
  let inited = useRef(false)
  useEffect(() => {
    if (inited.current) {
      setClient(new Client({ onError, session }))
    } else {
      inited.current = true
    }
  }, [session])
  return (
    <ClientContext.Provider value={client}>{children}</ClientContext.Provider>
  )
}

export function useClient() {
  const client = useContext(ClientContext)
  if (!client)
    throw new Error(
      `useClient must be called within a component wrapped by ClientProvider`
    )
  return client
}

type AuthState = { token?: Nullable<string> }
