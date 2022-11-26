import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../../components/layout"
import { SEO } from "../../../components/seo"
import { useTranslateScoped } from "../../../hooks/translate"
import { useClient } from "../../../providers/client"

export default function () {
  const translate = useTranslate()
  const client = useClient()
  return <Layout title={translate(`accountTitle`)}>login form</Layout>
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
