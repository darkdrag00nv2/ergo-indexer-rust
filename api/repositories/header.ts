import { option } from "../deps.ts";
import { Header } from "../types/indexer/header.ts";
import { DbPool } from "./db/db.ts";

export class HeaderRepo {
  constructor(private dbPool: DbPool) {}

  async getLast(): Promise<option.Option<Header>> {
    const conn = await this.dbPool.get_connection();

    const result = await conn.queryObject<Header>(
      {
        text:
"SELECT id, parent_id, version, height, n_bits, difficulty, timestamp, state_root, \
            ad_proofs_root, transactions_root, extension_hash, miner_pk, w, n, d, votes, main_chain \
            from node_headers where main_chain = true order by height desc limit 1",
        camelcase: true,
      },
    );

    if (result.rowCount && result.rowCount == 1) {
      return option.some(result.rows[0]);
    } else {
      return option.none;
    }
  }
}
