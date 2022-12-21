import { zodResolver } from "@hookform/resolvers/zod"
import { Button, Grid, TextField } from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Layout } from "../../../components/layout"
import { SEO } from "../../../components/seo"
import { useTranslateScoped } from "../../../hooks/translate"
import { useClient } from "../../../providers/client"
import { useSession } from "../../../providers/session"

export default function () {
  const translate = useTranslate()
  const client = useClient()
  const [, setSession] = useSession()
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<SignUpInput>({
    mode: `onChange`,
    resolver: zodResolver(
      z
        .object({ name: z.string().min(1), password: z.string().min(5) })
        .strict() as z.Schema<SignUpInput>
    ),
  })
  return (
    <Layout title={translate(`accountTitle`)}>
      <Grid
        container
        direction="column"
        component="form"
        alignItems="center"
        spacing={2}
        onSubmit={handleSubmit(async vals => {
          const session = await client?.signup(vals)
          if (session) {
            setSession(session)
            window.history.back()
          }
        })}
      >
        <Grid item>
          <TextField
            autoFocus
            required
            disabled={isSubmitting}
            label="Username"
            {...register(`name`)}
            error={!!errors.name}
            helperText={errors.name?.message}
          />
        </Grid>
        <Grid item>
          <TextField
            required
            disabled={isSubmitting}
            type="password"
            label="Password"
            {...register(`password`)}
            error={!!errors.password}
            helperText={errors.password?.message}
          />
        </Grid>
        <Grid item>
          <Button type="submit" disabled={isSubmitting} variant="contained">
            sign up
          </Button>
        </Grid>
      </Grid>
    </Layout>
  )
}

function useTranslate() {
  return useTranslateScoped(`settings`)
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
