import { Button, Grid, TextField } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useI18next } from "gatsby-plugin-react-i18next"
import { classValidatorResolver } from "@hookform/resolvers/class-validator"
import { useCallback } from "react"
import { Controller, useForm } from "react-hook-form"
import { useTranslation } from "react-i18next"
import { IsString } from "class-validator"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { useSession } from "../hooks/session"
import { useClient } from "../providers/client"

class SignupInput {
  @IsString()
  name!: string
  @IsString()
  password!: string
}

export default function () {
  const { t } = useTranslation()
  const { navigate } = useI18next()
  const translate = useCallback((key: string) => t(key, { ns: `signup` }), [t])
  const client = useClient()
  const [, setSession] = useSession()
  const { control, handleSubmit } = useForm<{ name: string; password: string }>(
    { resolver: classValidatorResolver(SignupInput) }
  )
  return (
    <Layout title={translate(`title`)}>
      <Grid container justifyContent="center">
        <Grid
          item
          component="form"
          onSubmit={handleSubmit(async vals => {
            alert(JSON.stringify(vals))
            // setSession(await client.signup(vals))
            // window.history.back()
          })}
        >
          <Grid container direction="column" spacing={2} alignItems="center">
            <Grid item>
              <Controller
                name="name"
                control={control}
                render={({ field }) => <TextField label="Name" {...field} />}
              />
            </Grid>
            <Grid item>
              <Controller
                name="password"
                control={control}
                render={({ field }) => (
                  <TextField label="Password" {...field} />
                )}
              />
            </Grid>
            <Grid item>
              <Button type="submit">Sign Up</Button>
            </Grid>
          </Grid>
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
