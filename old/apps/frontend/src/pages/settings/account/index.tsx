import { graphql, HeadFC } from "gatsby"
import { useEffect, useState } from "react"

import { Layout } from "../../../components/layout"
import { ListSection, ListSectionItem } from "../../../components/listSection"
import { Loading } from "../../../components/loading"
import { SEO } from "../../../components/seo"
import { useTranslateScoped } from "../../../hooks/translate"
import { useClient } from "../../../providers/client"
import * as routes from "../../../routes"

export default function () {
  const translate = useTranslate()
  const client = useClient()
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)
  useEffect(() => {
    let active = true
    ;(async () => {
      if (client && active) {
        try {
          setLoading(true)
          const user = await client.getSelf()
          if (active && user) setUser(user)
        } finally {
          setLoading(false)
        }
      }
    })()
    return () => {
      active = false
    }
  }, [client])
  return (
    <Layout title={translate(`accountTitle`)}>
      {loading ? (
        <Loading />
      ) : (
        <ListSection title="Account">
          {user ? (
            <>
              <ListSectionItem title="Username" value={user?.name} />
              <ListSectionItem title="Email" value={user?.email} />
            </>
          ) : (
            <>
              <ListSectionItem
                title="Log in"
                to={routes.LOGIN_ACCOUNT_SETTINGS}
              />
              <ListSectionItem
                title="Sign up"
                to={routes.SIGNUP_ACCOUNT_SETTINGS}
              />
            </>
          )}
        </ListSection>
      )}
    </Layout>
  )
}

function useTranslate() {
  return useTranslateScoped(`settings`)
}

export const Head: HeadFC = () => <SEO />

export const query = graphql`
  query ($language: String!) {
    locales: allLocale(filter: { language: { eq: $language } }) {
      edges {
        node {
          ns
          data
          language
        }
      }
    }
  }
`
