import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { useTranslateScoped } from "../../hooks/translate"

export default function () {
  const translate = useTranslate()
  return <Layout title={translate(`securityTitle`)}>sync</Layout>
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
