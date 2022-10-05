import { Grid } from "@mui/material"
import { StaticImage } from "gatsby-plugin-image"
import { graphql, HeadFC } from "gatsby"
import { ComponentProps, PropsWithChildren } from "react"

import * as routes from "../routes"
import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { Tile } from "../components/tile"

export default function () {
  const iconStyles: Omit<ComponentProps<typeof StaticImage>, "alt" | "src"> = {
    placeholder: "blurred",
    layout: "fixed",
    width: 80,
    height: 80,
  }
  return (
    <Layout isPrivate>
      <Grid container alignItems="stretch" spacing={2}>
        <Item>
          <Tile
            title="Notes"
            description="Record anything"
            link={routes.NOTES}
            icon={
              <StaticImage
                alt="notes"
                src="../images/notes-icon.png"
                {...iconStyles}
              />
            }
          />
        </Item>
        <Item>
          <Tile
            title="Reports"
            description="View your weekly report"
            link={routes.REPORTS}
            icon={
              <StaticImage
                alt="reports"
                src="../images/reports-icon.png"
                {...iconStyles}
              />
            }
          />
        </Item>
      </Grid>
    </Layout>
  )
}

function Item({ children }: PropsWithChildren) {
  return (
    <Grid item xs={12} sm={6} lg={4}>
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
