import { zodResolver } from "@hookform/resolvers/zod"
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Grid,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  SpeedDial,
  SpeedDialIcon,
  TextField,
  useTheme,
} from "@mui/material"
import { graphql, HeadFC } from "gatsby"
import { useEffect } from "react"
import { useState } from "react"
import { useForm } from "react-hook-form"
import ReactMarkdown from "react-markdown"
import { z } from "zod"

import { Dashboard } from "../../components/dashboard"
import { Layout } from "../../components/layout"
import { Loading } from "../../components/loading"
import { SEO } from "../../components/seo"
import { Tile } from "../../components/tile"
import { useTranslateScoped } from "../../hooks/translate"
import { useClient } from "../../providers/client"
import { DeleteForever, Edit } from "@mui/icons-material"

export default function () {
  const translate = useTranslate()
  const [creating, setCreating] = useState(false)
  const [memos, setMemos] = useState<Memo[]>([])
  const [reloading, setReloading] = useState(true)
  const client = useClient()
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<CreateMemoInput>({
    resolver: zodResolver(
      z.object({ content: z.string() }).strict() as z.Schema<CreateMemoInput>
    ),
  })
  useEffect(() => {
    let active = true
    if (reloading)
      client.listMemos().then(memos => {
        if (active && memos) setMemos(memos.edges.map(v => v.node))
        setReloading(false)
      })
    return () => {
      active = false
    }
  }, [reloading])
  return (
    <Layout title={translate(`title`)} isPrivate>
      {reloading && <Loading />}
      <Dashboard>
        <Grid container alignItems="stretch" spacing={2}>
          {memos.map(memo => (
            <Grid item key={memo.id} xs={12} sm={6} md={4} lg={3}>
              <Item
                memo={memo}
                onChanged={() => {
                  setReloading(true)
                }}
              />
            </Grid>
          ))}
        </Grid>
      </Dashboard>
      <SpeedDial
        open={false}
        ariaLabel="Create memo"
        onClick={() => setCreating(true)}
        icon={<SpeedDialIcon />}
        sx={{ position: `absolute`, bottom: 75, right: 20 }}
      />
      <Dialog open={creating} onClose={() => setCreating(false)} keepMounted>
        <form
          onSubmit={handleSubmit(async vals => {
            await client.createMemo(vals)
            setCreating(false)
            setReloading(true)
          })}
        >
          <DialogTitle>Create Memo</DialogTitle>
          <DialogContent>
            <TextField
              label="Content"
              error={!!errors.content}
              helperText={errors.content?.message}
              fullWidth
              multiline
              minRows={5}
              disabled={isSubmitting}
              {...register(`content`)}
            />
          </DialogContent>
          <DialogActions>
            <Button color="error" onClick={() => setCreating(false)}>
              Cancel
            </Button>
            <Button type="submit">Save</Button>
          </DialogActions>
        </form>
      </Dialog>
    </Layout>
  )
}

function Item({ memo, onChanged }: { memo: Memo; onChanged: () => void }) {
  const [editing, setEditing] = useState(false)
  const [updating, setUpdating] = useState(false)
  const [deleting, setDeleting] = useState(false)
  const client = useClient()
  const theme = useTheme()
  const deleteColor = theme.palette.secondary.light
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<UpdateMemoInput>({
    resolver: zodResolver(
      z.object({
        content: z.string().optional(),
      }) as z.Schema<UpdateMemoInput>
    ),
    defaultValues: { ...memo },
  })
  return (
    <>
      <Tile linkText="Edit" onClick={() => setEditing(true)}>
        <ReactMarkdown>{memo.content}</ReactMarkdown>
      </Tile>
      <Dialog keepMounted open={editing} onClose={() => setEditing(false)}>
        <List>
          <ListItemButton
            onClick={() => {
              setUpdating(true)
              setEditing(false)
            }}
          >
            <ListItemIcon>
              <Edit />
            </ListItemIcon>
            <ListItemText>Update</ListItemText>
          </ListItemButton>
          <ListItemButton
            onClick={() => {
              setDeleting(true)
              setEditing(false)
            }}
          >
            <ListItemIcon color={deleteColor}>
              <DeleteForever />
            </ListItemIcon>
            <ListItemText>
              <div color={deleteColor}>Delete</div>
            </ListItemText>
          </ListItemButton>
        </List>
      </Dialog>
      <Dialog open={updating}>
        <form
          onSubmit={handleSubmit(async vals => {
            await client.updateMemo(memo.id, vals)
            setUpdating(false)
            onChanged()
          })}
        >
          <DialogTitle>Update Memo</DialogTitle>
          <DialogContent>
            <TextField
              label="Content"
              fullWidth
              multiline
              minRows={5}
              disabled={isSubmitting}
              {...register(`content`)}
              error={!!errors.content}
              helperText={errors.content?.message}
            />
          </DialogContent>
          <DialogActions>
            <Button
              color="error"
              disabled={isSubmitting}
              onClick={() => {
                setUpdating(false)
              }}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              Save
            </Button>
          </DialogActions>
        </form>
      </Dialog>
      <Dialog
        open={deleting}
        keepMounted
        onClose={() => {
          setDeleting(false)
        }}
      >
        <DialogTitle>Delete Memo</DialogTitle>
        <DialogContent>
          This action cannot be reverted. Are you sure?
        </DialogContent>
        <DialogActions>
          <Button
            onClick={() => {
              setDeleting(false)
            }}
          >
            No
          </Button>
          <Button
            color="warning"
            onClick={async () => {
              await client.deleteMemo(memo.id)
              setDeleting(false)
              onChanged()
            }}
          >
            Yes
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}

function useTranslate() {
  return useTranslateScoped(`memo`)
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
