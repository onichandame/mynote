import { zodResolver } from "@hookform/resolvers/zod"
import {
  Button,
  Card,
  CardActions,
  CardContent,
  Container,
  TextField,
} from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Layout } from "../../components/layout"
import { SEO } from "../../components/seo"
import { Tile } from "../../components/tile"
import { useTranslateScoped } from "../../hooks/translate"
import { useClient } from "../../providers/client"
import { useCurrentUser } from "../../providers/currentUser"

export default function () {
  const translate = useTranslate()
  const { reload, user } = useCurrentUser()
  const client = useClient()
  const {
    register,
    handleSubmit,
    formState: { isSubmitting, errors },
  } = useForm<FormValues>({
    mode: `onChange`,
    resolver: zodResolver(
      z.object({ name: z.string().min(1) }).strict() as z.Schema<FormValues>
    ),
  })
  return (
    <Layout title={translate(`profileBasicInfoName`)}>
      <form
        onSubmit={handleSubmit(async vals => {
          await client?.updateSelf(vals)
          reload()
          window.history.back()
        })}
      >
        <Tile
          actions={
            <>
              <Button
                color="warning"
                disabled={isSubmitting}
                onClick={() => {
                  window.history.back()
                }}
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
            autoFocus
            label="Username"
            defaultValue={user?.name}
            error={!!errors.name}
            helperText={errors.name?.message}
            {...register(`name`)}
            disabled={isSubmitting}
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

type FormValues = Pick<UpdateUserInput, "name">
