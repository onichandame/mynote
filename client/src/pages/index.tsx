import { graphql, HeadFC } from "gatsby"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"

const IndexPage = () => {
  return <Layout>home</Layout>
}

export default IndexPage

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
