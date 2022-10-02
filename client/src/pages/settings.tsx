import { List } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useTranslation } from "gatsby-plugin-react-i18next"
import { useCallback } from "react"

import { Layout } from "../components/layout"
import { NavigationItem } from "../components/navigationItem"
import { SEO } from "../components/seo"

export default function () {
  const { t } = useTranslation()
  const translate = useCallback(
    (key: string) => t(key, { ns: `settings` }),
    [t]
  )
  return (
    <Layout title={translate(`title`)} isPrivate>
      <List>
        <NavigationItem title="Profile" to="./profile" />
      </List>
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
