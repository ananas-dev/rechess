import type { Key } from "chessground/types";

const toDests = (destObject): Map<Key, Key[]> =>
  new Map(
    Object.entries(destObject).map((entry: [string, string]) => [
      entry[0] as Key,
      entry[1].match(/.{1,2}/g) as Key[],
    ])
  );

export { toDests };
