import { useCallback } from "react"
import { useTranslation } from "react-i18next"

export function useTranslateScoped(scope: string) {
  const { t } = useTranslation()
  const translate = useCallback(
    (key: string) => t(key, { ns: scope }),
    [scope, t]
  )
  return translate
}
