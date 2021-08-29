import type { Key } from "chessground/types";

const toDests = (destObject): Map<Key, Key[]> =>
  new Map(
    Object.entries(destObject).map((entry: [string, string]) => [
      entry[0] as Key,
      entry[1].match(/.{1,2}/g) as Key[],
    ])
  );

const uciToMove = (
  uci: string
): { orig: Key; dest: Key; promotion?: string } => {
  return {
    orig: uci.slice(0, 2) as Key,
    dest: uci.slice(2, 4) as Key,
    promotion: uci.length == 5 ? uci[4] : null,
  };
};

const moveToUci = (move: {
  orig: Key;
  dest: Key;
  promotion?: string;
}): string => move.orig + move.dest + (move.promotion ?? "");

export { toDests, uciToMove, moveToUci };
