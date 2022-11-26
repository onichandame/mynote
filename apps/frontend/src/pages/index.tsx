import { Grid } from "@mui/material"
import { StaticImage } from "gatsby-plugin-image"
import { graphql, HeadFC } from "gatsby"
import { ComponentProps, PropsWithChildren } from "react"

import * as routes from "../routes"
import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { Tile } from "../components/tile"
import { useTranslateScoped } from "../hooks/translate"

export default function () {
  const translate = useTranslate()
  const iconStyles: Omit<ComponentProps<typeof StaticImage>, "alt" | "src"> = {
    placeholder: "blurred",
    layout: "fixed",
    width: 80,
    height: 80,
  }
  return (
    <Layout>
      <Grid container alignItems="stretch" spacing={2}>
        <Item>
          <Tile
            title={translate(`memoTitle`)}
            description={translate(`memoDescription`)}
            link={routes.MEMO}
            icon={
              <StaticImage
                alt="memo"
                src="../images/memo-icon.png"
                {...iconStyles}
              />
            }
          />
        </Item>
        <Item>
          <Tile
            title={translate(`reportTitle`)}
            description={translate(`reportDescription`)}
            link={routes.REPORT}
            icon={
              <StaticImage
                alt="reports"
                src="../images/report-icon.png"
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

function useTranslate() {
  return useTranslateScoped(`home`)
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
