import { Filter } from "./filter";

export interface Note {
  id: number;
  title: string;
  content: string;
  createdAt: Date;
  updatedAt?: Date;
  deletedAt?: Date;
}

export interface NoteFilter {
  deleted_at?: Filter<Date>;
}
