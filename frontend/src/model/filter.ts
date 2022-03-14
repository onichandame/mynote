export interface Filter<TData> {
  eq?: TData;
  null?: boolean;
  lt?: TData;
  lte?: TData;
  gt?: TData;
  gte?: TData;
  like?: string;
  and?: Filter<TData>[];
  or?: Filter<TData>[];
  not?: boolean;
}
