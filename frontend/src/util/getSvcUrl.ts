export const getSvcUrl = () =>
  window.location.protocol.startsWith(`https`)
    ? `wss://`
    : `ws://` + window.location.host + `/graphql`;
