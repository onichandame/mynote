import { AccountCircle, Lock } from "@mui/icons-material"
import { Grid } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { PropsWithChildren } from "react"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { Tile } from "../../components/tile"
import { useTranslateScoped } from "../../hooks/translate"
import * as routes from "../../routes"

export default function () {
  const translate = useTranslateScoped(`settings`)
  return (
    <Layout title={translate(`title`)} isPrivate>
      <Grid container spacing={2} alignItems="stretch">
        <Item>
          <Tile
            title={translate(`profileTitle`)}
            description={translate(`profileDescription`)}
            icon={<AccountCircle fontSize="large" color="info" />}
            linkText={translate(`profileLinkText`)}
            link={routes.PROFILE_SETTINGS}
          />
        </Item>
        <Item>
          <Tile
            title={translate(`securityTitle`)}
            description={translate(`securityDescription`)}
            icon={<Lock fontSize="large" color="success" />}
            linkText={translate(`securityLinkText`)}
            link={routes.SECURITY_SETTINGS}
          />
        </Item>
      </Grid>
    </Layout>
  )
}

function Item({ children }: PropsWithChildren) {
  return (
    <Grid item xs={12} sm={6} md={4} lg={3}>
      {children}
    </Grid>
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
