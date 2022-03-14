export interface Sorting {
  field: string;
  direction: `ASC` | `DESC`;
}

export type Sortings = Sorting[];
