import { graphql, HeadFC } from "gatsby"

import { Dashboard } from "../../components/dashboard"
import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { useTranslateScoped } from "../../hooks/translate"

export default function () {
  const translate = useTranslate()
  return (
    <Layout title={translate(`title`)} isPrivate>
      <Dashboard>To be continued...</Dashboard>
    </Layout>
  )
}

function useTranslate() {
  return useTranslateScoped(`report`)
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
