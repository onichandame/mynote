import { Box } from "@mui/material"
import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../components/layout"
import { ListSection, ListSectionItem } from "../../components/listSection"
import { SEO } from "../../components/seo"
import { useTranslateScoped } from "../../hooks/translate"
import * as routes from "../../routes"

export default function () {
  const translate = useTranslate()
  return (
    <Layout title={translate(`securityTitle`)}>
      <Box sx={{ padding: 2 }}>
        <ListSection title="Account">
          <ListSectionItem
            title={translate(`securityAccountPassword`)}
            to={routes.PASSWORD_SETTINGS}
          />
        </ListSection>
      </Box>
    </Layout>
  )
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
