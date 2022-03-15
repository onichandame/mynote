export const formatError = (e: unknown): string => {
  return Array.isArray(e)
    ? e.map((v) => formatError(v)).join(`; `)
    : e instanceof Error
    ? e.message
    : JSON.stringify(e);
};
