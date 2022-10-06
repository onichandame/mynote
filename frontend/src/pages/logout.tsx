import { graphql, HeadFC } from "gatsby"
import { useI18next } from "gatsby-plugin-react-i18next"
import { useEffect } from "react"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { useTranslateScoped } from "../hooks/translate"
import { useSession } from "../providers/session"

export default function () {
  const [, setSession] = useSession()
  const { navigate } = useI18next()
  const translate = useTranslateScoped(`logout`)
  useEffect(() => {
    setSession(null, true)
    navigate(`/`, { replace: true })
  }, [setSession])
  return (
    <Layout title={translate(`title`)} isPrivate>
      {translate(`body`)}
    </Layout>
  )
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
