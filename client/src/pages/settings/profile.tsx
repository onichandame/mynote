import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { useTranslateScoped } from "../../hooks/translate"

export default function () {
  const translate = useTranslateScoped(`settings`)
  return (
    <Layout title={translate(`profileTitle`)} isPrivate>
      profile settings
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
