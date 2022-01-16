import { Note as FullNote } from "../type";

export type Note = Pick<
  FullNote,
  "id" | "createdAt" | "updatedAt" | "title" | "content"
>;
