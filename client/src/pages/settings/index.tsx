import { AccountCircle } from "@mui/icons-material"
import { Grid } from "@mui/material"
import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { Tile } from "../../components/tile"
import { useTranslateScoped } from "../../hooks/translate"
import * as routes from "../../routes"

export default function () {
  const translate = useTranslateScoped(`settings`)
  return (
    <Layout title={translate(`title`)} isPrivate>
      <Grid container padding={2} spacing={2}>
        <Grid item xs={12} sm={6} md={4} lg={3}>
          <Tile
            title={translate(`profileTitle`)}
            description={translate(`profileDescription`)}
            icon={
              <AccountCircle
                sx={{
                  fontSize: `3rem`,
                  color: theme => theme.palette.success.dark,
                }}
              />
            }
            linkText={translate(`profileLinkText`)}
            link={routes.PROFILE_SETTINGS}
          />
        </Grid>
      </Grid>
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
