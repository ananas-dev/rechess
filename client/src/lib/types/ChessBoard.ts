import type { Api } from "chessground/api";
import type { MoveMetadata, Key } from "chessground/types";

export interface MoveEvent {
  from: Key;
  to: Key;
  metadata: MoveMetadata;
  cg: Api;
}
