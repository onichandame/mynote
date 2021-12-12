import { Button, FormHelperText, TextField } from '@mui/material'
import { useQuery } from '@onichandame/react-graphql-ws'
import * as yup from 'yup'
import { FC, useContext, useState } from 'react'
import { useFormik } from 'formik'

import { SessionContext } from '../auth'
import { useClient } from '../fetcher'

export const Login: FC = () => {
  const [, setSession] = useContext(SessionContext)
  const client = useClient()
  const [errmsg, setErrMsg] = useState(``)
  const login = useQuery<{ login: string }, { name: string; password: string }>(
    {
      query: `mutation ($name: String!, $password: String!) {login(name: $name, password: $password)}`,
      client,
    }
  )
  const schema = yup.object({
    name: yup.string().required().min(1).default(``),
    password: yup.string().required().min(5).default(``),
  })
  const form = useFormik<yup.Asserts<typeof schema>>({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true)
      try {
        const res = await login(vals)
        if (res.errors)
          throw new Error(
            res.errors.reduce(
              (prev, curr) => [prev, curr.message].join(`\n`),
              ``
            )
          )
        if (!res.data?.login) throw new Error(`failed to receive session token`)
        window.localStorage.setItem(`mynote_session`, res.data.login)
        setSession(res.data.login)
        setErrMsg(``)
      } catch (e: unknown) {
        setErrMsg(e instanceof Error ? e.message : JSON.stringify(e))
      }
      helpers.setSubmitting(false)
    },
  })
  return (
    <form onSubmit={form.handleSubmit}>
      <TextField
        fullWidth
        name="name"
        value={form.values.name}
        onBlur={form.handleBlur}
        helperText={form.errors.name}
        error={!!form.errors.name}
        onChange={form.handleChange}
        sx={{ marginBottom: theme => theme.spacing(3) }}
      />
      <TextField
        fullWidth
        name="password"
        value={form.values.password}
        onBlur={form.handleBlur}
        helperText={form.errors.password}
        error={!!form.errors.password}
        onChange={form.handleChange}
        sx={{ marginBottom: theme => theme.spacing(3) }}
      />
      <FormHelperText
        error={!!errmsg}
        sx={{ marginBottom: theme => theme.spacing(3) }}
      >
        {errmsg}
      </FormHelperText>
      <Button color="primary" type="submit">
        login
      </Button>
    </form>
  )
}
