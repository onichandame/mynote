import { Button, TextField } from '@mui/material'
import * as yup from 'yup'
import { FC } from 'react'
import { useFormik } from 'formik'
import { useNavigate } from 'react-router'

import { db } from '../db'

export const CreateNote: FC = () => {
  const nav = useNavigate()
  const schema = yup.object({
    title: yup.string().min(1).default(``).required(),
    content: yup.string().min(1).default(``).required(),
  })
  const form = useFormik<yup.Asserts<typeof schema>>({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true)
      await db.notes.add({ ...vals, createdAt: new Date() })
      helpers.setSubmitting(false)
      nav('/')
    },
  })
  return (
    <form onSubmit={form.handleSubmit}>
      <TextField
        placeholder="Title"
        fullWidth
        onChange={form.handleChange}
        name="title"
        value={form.values.title}
        sx={{ marginBottom: theme => theme.spacing(3) }}
      />
      <TextField
        placeholder="Content"
        multiline
        fullWidth
        name="content"
        value={form.values.content}
        onChange={form.handleChange}
        sx={{ marginBottom: theme => theme.spacing(3) }}
      />
      <Button
        variant="contained"
        disabled={form.isSubmitting}
        type="submit"
        fullWidth
        sx={{ marginBottom: theme => theme.spacing(3) }}
      >
        Create Note
      </Button>
    </form>
  )
}
