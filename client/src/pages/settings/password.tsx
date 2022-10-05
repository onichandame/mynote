import { zodResolver } from "@hookform/resolvers/zod"
import {
  Button,
  Card,
  CardActions,
  CardContent,
  TextField,
} from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useI18next } from "gatsby-plugin-react-i18next"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { Tile } from "../../components/tile"
import { useTranslateScoped } from "../../hooks/translate"
import { useClient } from "../../providers/client"
import { useCurrentUser } from "../../providers/currentUser"
import * as routes from "../../routes"

export default function () {
  const translate = useTranslate()
  const { navigate } = useI18next()
  const client = useClient()
  const { reload } = useCurrentUser()
  const {
    register,
    handleSubmit,
    formState: { isSubmitting, errors },
  } = useForm<ChangePasswordInput>({
    mode: `onChange`,
    resolver: zodResolver(
      z.object({ password: z.string().min(5) }) as z.Schema<ChangePasswordInput>
    ),
  })
  return (
    <Layout title={translate(`securityAccountPassword`)}>
      <form
        onSubmit={handleSubmit(async vals => {
          await client.changePassword(vals)
          reload()
          navigate(routes.HOME)
        })}
      >
        <Tile
          actions={
            <>
              <Button
                color="warning"
                disabled={isSubmitting}
                onClick={() => window.history.back()}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={isSubmitting}>
                Save
              </Button>
            </>
          }
        >
          <TextField
            label="New Password"
            type="password"
            {...register(`password`)}
            disabled={isSubmitting}
            error={!!errors.password}
            helperText={errors.password?.message}
          />
        </Tile>
      </form>
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
