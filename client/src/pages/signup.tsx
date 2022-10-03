import { Button, Grid, TextField } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useI18next } from "gatsby-plugin-react-i18next"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Layout } from "../components/layout"
import { SEO } from "../components/seo"
import { useClient } from "../providers/client"
import { useSession } from "../providers/session"
import { useTranslateScoped } from "../hooks/translate"

export default function () {
  const { navigate } = useI18next()
  const translate = useTranslateScoped(`signup`)
  const client = useClient()
  const [, setSession] = useSession()
  const {
    register,
    handleSubmit,
    formState: { isSubmitting, errors },
  } = useForm<SignUpInput>({
    mode: `onChange`,
    resolver: zodResolver(
      z
        .object({ name: z.string().min(1), password: z.string().min(5) })
        .strict() as z.Schema<SignUpInput>
    ),
    defaultValues: { name: ``, password: `` },
  })
  return (
    <Layout publicOnly title={translate(`title`)}>
      <Grid container justifyContent="center">
        <Grid
          item
          component="form"
          onSubmit={handleSubmit(async vals => {
            const session = await client.signup(vals)
            if (session) {
              setSession(session)
              navigate(`/`, { replace: true })
            }
          })}
        >
          <Grid container direction="column" spacing={2} alignItems="center">
            <Grid item>
              <TextField
                {...register(`name`)}
                error={!!errors.name}
                helperText={errors.name?.message}
                autoFocus
                label="Name"
              />
            </Grid>
            <Grid item>
              <TextField
                {...register(`password`)}
                error={!!errors.password}
                helperText={errors.password?.message}
                label="Password"
              />
            </Grid>
            <Grid item>
              <Button type="submit" variant="contained" disabled={isSubmitting}>
                {translate(`buttonText`)}
              </Button>
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
