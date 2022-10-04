import { graphql, HeadFC } from "gatsby"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { useTranslateScoped } from "../hooks/translate"

export default function NotFoundPage() {
  const translate = useTranslateScoped(`404`)
  return <Layout title={translate(`title`)}>404 | Page not found</Layout>
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
