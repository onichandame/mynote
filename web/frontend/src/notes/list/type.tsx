import { Note as FullNote } from "../type";

export type Note = Pick<FullNote, "title" | "id" | "createdAt" | "content">;
