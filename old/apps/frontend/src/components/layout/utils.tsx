import { useTranslateScoped } from "../../hooks/translate"

export function useTranslate() {
  return useTranslateScoped(`layout`)
}
