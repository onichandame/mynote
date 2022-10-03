import { graphql, HeadFC } from "gatsby"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"

export default function () {
  return <Layout>home</Layout>
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
