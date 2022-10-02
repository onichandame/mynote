import { graphql, HeadFC } from "gatsby"
import { Layout } from "../components/layout"
import { SEO } from "../components/seo"

const NotFoundPage = () => {
  return <Layout title="404 Not Found">404</Layout>
}

export default NotFoundPage

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
