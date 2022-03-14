export type Note = {
  id: number;
  createdAt: Date;
  updatedAt?: Date;
  deletedAt?: Date;

  title: string;
  content: string;
};
