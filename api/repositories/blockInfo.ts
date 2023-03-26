import { DbPool } from "./db/db.ts";

export class BlockInfoRepo {
  constructor(private dbPool: DbPool) {}

  async totalDifficultySince(_ts: number): Promise<number> {
    return 0;
  }
}
