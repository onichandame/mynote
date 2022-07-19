import type { GatsbyConfig } from "gatsby"

const trackingId = process.env.GA_TRACKING_ID
const siteUrl = process.env.SITE_URL

const config: GatsbyConfig = {
  siteMetadata: {
    siteUrl,
  },
  // More easily incorporate content into your pages through automatic TypeScript type generation and better GraphQL IntelliSense.
  // If you use VSCode you can also use the GraphQL plugin
  // Learn more at: https://gatsby.dev/graphql-typegen
  graphqlTypegen: true,
  plugins: [
    "gatsby-plugin-emotion",
    trackingId && {
      resolve: "gatsby-plugin-google-analytics",
      options: {
        trackingId,
      },
    },
    "gatsby-plugin-sitemap",
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        icon: "src/images/icon.png",
      },
    },
  ].filter(isNotEmpty),
}

export default config

function isNotEmpty<T>(raw: T): raw is NonNullable<T> {
  return !!raw
}
