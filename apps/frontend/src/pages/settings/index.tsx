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
    <Layout title={translate(`title`)}>
      <Grid container spacing={2} alignItems="stretch">
        <Item>
          <Tile
            title={translate(`accountTitle`)}
            description={translate(`accountDescription`)}
            icon={<AccountCircle fontSize="large" color="info" />}
            linkText={translate(`accountLinkText`)}
            link={routes.ACCOUNT_SETTINGS}
          />
        </Item>
        <Item>
          <Tile
            title={translate(`syncTitle`)}
            description={translate(`syncDescription`)}
            icon={<Lock fontSize="large" color="success" />}
            linkText={translate(`syncLinkText`)}
            link={routes.SYNC_SETTINGS}
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
