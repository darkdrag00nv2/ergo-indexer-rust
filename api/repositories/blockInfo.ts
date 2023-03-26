import { DbPool } from "./db/db.ts";

export class BlockInfoRepo {
  constructor(private dbPool: DbPool) {}
}
