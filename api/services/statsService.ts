import { option } from "../deps.ts";
import {
  BlockInfoRepo,
  HeaderRepo,
  TransactionRepo,
} from "../repositories/repositories.ts";
import { Header } from "../types/indexer/header.ts";
import { BlockChainInfo } from "../types/indexer/info.ts";

export class StatsService {
  private secondsIn24H = (24 * 60 * 60);
  private millisIn24H = this.secondsIn24H * 1000;

  constructor(
    private headerRepo: HeaderRepo,
    private blockInfoRepo: BlockInfoRepo,
    private transactionRepo: TransactionRepo,
  ) {}

  async getBlockChainInfo(): Promise<BlockChainInfo> {
    const ts = this.getPastPeriodTsMillis();
    const [headerOpt, diff, numTxs] = await Promise.all([
      this.headerRepo.getLast(),
      this.blockInfoRepo.totalDifficultySince(ts),
      this.transactionRepo.countMainSince(ts),
    ]);
    const hashRate = this.dailyHashRate(diff);

    return option.match<Header, BlockChainInfo>(
      () => ({
        version: "0.0.0",
        supply: 0,
        transactionAverage: 0,
        hashRate: 0,
      }),
      (h) => ({
        version: h.version.toString(),
        supply: 1,
        transactionAverage: numTxs,
        hashRate: hashRate,
      }),
    )(headerOpt);
  }

  getPastPeriodTsMillis(): number {
    const date = new Date();
    return date.getTime() - this.millisIn24H;
  }

  dailyHashRate(difficulty: number) {
    return (difficulty / this.secondsIn24H) + 1;
  }
}
