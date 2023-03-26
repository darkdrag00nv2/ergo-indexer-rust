import { DbPool } from "./db/db.ts";

export class TransactionRepo {
  constructor(private dbPool: DbPool) {}

  /// Get total number of transactions appeared in the main chain after a given timestamp ts.
  async countMainSince(ts: number): Promise<number> {
    const conn = await this.dbPool.get_connection();

    const result = await conn.queryObject<
      { count_main: number }
    >`select count(id) as count_main from node_transactions where timestamp >= ${ts}`;

    if (result.rowCount && result.rowCount == 1) {
      return result.rows[0].count_main;
    } else {
      throw new Error(
        "Error in countMainSince, expected exactly one row",
      );
    }
  }
}
