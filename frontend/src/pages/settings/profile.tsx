import { Container, Grid } from "@mui/material"
import { graphql, HeadFC } from "gatsby"

import { Layout } from "../../components/layout"
import { CurrentAvatar } from "../../components/currentAvatar"
import { ListSection, ListSectionItem } from "../../components/listSection"
import { SEO } from "../../components/seo"
import { useTranslateScoped } from "../../hooks/translate"
import { useCurrentUser } from "../../providers/currentUser"
import * as routes from "../../routes"
import { useClient } from "../../providers/client"

export default function () {
  const translate = useTranslate()
  const { user } = useCurrentUser()
  return (
    <Layout title={translate(`profileTitle`)} isPrivate>
      <Grid container direction="column" spacing={2}>
        <Grid item>
          <ListSection title={translate(`profileBasicInfoTitle`)}>
            <AvatarItem />
            <ListSectionItem
              to={routes.NAME_SETTINGS}
              title={translate(`profileBasicInfoName`)}
              value={user?.name}
            />
          </ListSection>
        </Grid>
      </Grid>
    </Layout>
  )
}

function AvatarItem() {
  const translate = useTranslate()
  const client = useClient()
  const { reload } = useCurrentUser()
  return (
    <>
      <input
        type="file"
        id="avatar-selector"
        hidden
        onChange={async e => {
          const files = e.currentTarget.files
          if (files) {
            if (files.length > 1)
              throw new Error(`only one image can be selected`)
            const url = await client?.uploadFile(files[0]!)
            if (url) await client?.updateSelf({ avatar: url })
            reload()
          }
        }}
      />
      <label htmlFor="avatar-selector">
        <ListSectionItem
          title={translate(`profileBasicInfoAvatar`)}
          value={<CurrentAvatar />}
        />
      </label>
    </>
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
