import type { Api } from "chessground/api";
import type { MoveMetadata, Key } from "chessground/types";

export interface MoveEvent {
  orig: Key;
  dest: Key;
  metadata: MoveMetadata;
  cg: Api;
}
