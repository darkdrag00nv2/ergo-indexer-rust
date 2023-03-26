import { DbPool } from "./db/db.ts";

export class BlockInfoRepo {
  constructor(private dbPool: DbPool) {}

  async totalDifficultySince(ts: number): Promise<number> {
    const conn = await this.dbPool.get_connection();

    const result = await conn.queryObject<
      { difficulty: number }
    >`select coalesce(cast(sum(difficulty) as decimal), 0) as difficulty from blocks_info where timestamp >= ${ts}`;

    if (result.rowCount && result.rowCount == 1) {
      return result.rows[0].difficulty;
    } else {
      throw new Error(
        "Error in totalDifficultySince, expected exactly one row",
      );
    }
  }
}
