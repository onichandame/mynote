import { zodResolver } from "@hookform/resolvers/zod"
import { DeleteForever, Edit } from "@mui/icons-material"
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
import { Draggable, DraggableItemType } from "../../components/draggble"
import { useStore } from "../../providers/store"
import { Memo } from "../../providers/store/collections"

export default function () {
  const translate = useTranslate()
  const [creating, setCreating] = useState(false)
  const [memos, setMemos] = useState<Memo[]>([])
  const [reloading, setReloading] = useState(true)
  const store = useStore()
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<CreateInput>({
    resolver: zodResolver(
      z.object({ content: z.string() }).strict() as z.Schema<CreateInput>
    ),
  })
  useEffect(() => {
    let active = true
    ;(async () => {
      if (reloading && store) {
        try {
          const memos = await store.memos
            .find({ selector: { deletedAt: { $exists: false } } })
            .exec()
          if (active) setMemos(memos ?? [])
        } finally {
          setReloading(false)
        }
      }
    })()
    return () => {
      active = false
    }
  }, [reloading, store])
  return (
    <Layout title={translate(`title`)}>
      {reloading && <Loading />}
      <Dashboard>
        <Grid container alignItems="stretch" spacing={2}>
          {memos.map(memo => (
            <Grid item key={memo.id} xs={12} sm={6} md={4} lg={3}>
              <Draggable
                type={DraggableItemType.Memo}
                item={memo}
                onHover={(source, target) => {
                  if (source.id === target.id) return
                  const newMemos = [...memos]
                  const sourceIndex = newMemos.findIndex(
                    v => v.id === source.id
                  )
                  const sourceItem = newMemos.splice(sourceIndex, 1)[0]!
                  const targetIndex = newMemos.findIndex(
                    v => v.id === target.id
                  )
                  if (targetIndex >= 0) {
                    newMemos.splice(
                      sourceIndex > targetIndex ? targetIndex : targetIndex + 1,
                      0,
                      sourceItem
                    )
                    setMemos(newMemos)
                  }
                }}
                onDrop={async () => {
                  await Promise.all(
                    memos.map(async (memo, index) => {
                      if (memo.weight != index)
                        await memo.atomicPatch({ weight: index })
                    })
                  )
                  setReloading(true)
                }}
              >
                <Item
                  memo={memo}
                  onChanged={() => {
                    setReloading(true)
                  }}
                />
              </Draggable>
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
            await store?.memos.insert({
              ...vals,
            })
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
  const theme = useTheme()
  const deleteColor = theme.palette.secondary.light
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<UpdateInput>({
    resolver: zodResolver(
      z.object({
        content: z.string().optional(),
      }) as z.Schema<UpdateInput>
    ),
    defaultValues: memo.toJSON(),
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
            await memo.atomicPatch({ ...vals })
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
              await memo.softDelete()
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

type CreateInput = Pick<Memo, "content">
type UpdateInput = Pick<Partial<Memo>, "content" | "weight">
