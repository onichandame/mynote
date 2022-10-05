import { zodResolver } from "@hookform/resolvers/zod"
import { Button, Grid, TextField } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useI18next } from "gatsby-plugin-react-i18next"
import { Controller, useForm } from "react-hook-form"
import { z } from "zod"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { useTranslateScoped } from "../hooks/translate"
import { useClient } from "../providers/client"
import { useSession } from "../providers/session"
import * as routes from "../routes"

export default function () {
  const translate = useTranslateScoped(`login`)
  const { navigate } = useI18next()
  const client = useClient()
  const [, setSession] = useSession()
  const {
    register,
    handleSubmit,
    formState: { isSubmitting, errors },
  } = useForm<LogInInput>({
    mode: `onChange`,
    resolver: zodResolver(
      z
        .object({ identity: z.string().min(1), password: z.string().min(5) })
        .strict() as z.Schema<LogInInput>
    ),
  })
  return (
    <Layout publicOnly title={translate(`title`)}>
      <Grid
        container
        direction="column"
        alignItems="center"
        component="form"
        spacing={2}
        onSubmit={handleSubmit(async vals => {
          const session = await client.login(vals)
          if (session) {
            setSession(session)
            navigate(routes.HOME, { replace: true })
          }
        })}
      >
        <Grid item>
          <TextField
            required
            {...register(`identity`)}
            error={!!errors.identity}
            helperText={errors.identity?.message}
            autoFocus
            disabled={isSubmitting}
            label="Email/Username"
          />
        </Grid>
        <Grid item>
          <TextField
            required
            error={!!errors.password}
            helperText={errors.password?.message}
            {...register(`password`)}
            disabled={isSubmitting}
            label="Password"
          />
        </Grid>
        <Grid item>
          <Button type="submit" variant="contained" disabled={isSubmitting}>
            {translate(`buttonText`)}
          </Button>
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
