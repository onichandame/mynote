import type { GatsbyConfig } from "gatsby"

import * as dotenv from "dotenv"

dotenv.config({ path: `.env.${process.env.NODE_ENV}` })

const config: GatsbyConfig = {
  flags: { DEV_SSR: true },
  siteMetadata: {
    title: `Notebook`,
    siteUrl: `https://www.yourdomain.tld`,
  },
  jsxRuntime: `automatic`,
  // More easily incorporate content into your pages through automatic TypeScript type generation and better GraphQL IntelliSense.
  // If you use VSCode you can also use the GraphQL plugin
  // Learn more at: https://gatsby.dev/graphql-typegen
  graphqlTypegen: true,
  plugins: [
    `gatsby-plugin-image`,
    `gatsby-plugin-sharp`,
    `gatsby-transformer-sharp`, // Needed for dynamic images
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        path: `${__dirname}/locales`,
        name: `locale`,
      },
    },
    {
      resolve: `gatsby-plugin-react-i18next`,
      options: {
        localeJsonSourceName: `locale`, // name given to `gatsby-source-filesystem` plugin.
        languages: [`en`, `cn`],
        defaultLanguage: `en`,
        trailingSlash: "always",
        // you can pass any i18next options
        i18nextOptions: {
          interpolation: {
            escapeValue: false, // not needed for react as it escapes by default
          },
          keySeparator: false,
          nsSeparator: false,
        },
        pages: [],
      },
    },
    "gatsby-plugin-sitemap",
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        name: "Notebook",
        short_name: "Notebook",
        icon: "src/images/icon.png",
        start_url: "/",
        background_color: "#145ea8",
        display: `standalone`,
      },
    },
    `gatsby-plugin-offline`,
  ],
}

export default config
