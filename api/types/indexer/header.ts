import { BlockId, HexString } from "./common.ts";

export interface Header {
  id: BlockId;
  parentId: BlockId;
  version: number;
  height: number;
  nBits: number;
  difficulty: number;
  timestamp: number;
  stateRoot: HexString;
  adProofsRoot: HexString;
  transactionsRoot: HexString;
  extensionHash: HexString;
  minerPk: HexString;
  w: HexString; // PoW one time PK
  n: HexString; // PoW nonce
  d: string; // PoW distance
  votes: string; // hex-encoded votes for a soft-fork and parameters
  mainChain: boolean; // chain status, `true` if this header resides in main chain.
}
