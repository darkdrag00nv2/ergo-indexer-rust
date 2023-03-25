import { BlockInfoRepo, HeaderRepo } from "../repositories/repositories.ts";
import { BlockChainInfo } from "../types/indexer/info.ts";

export class StatsService {
  constructor(
    private headerRepo: HeaderRepo,
    private blockInfoRepo: BlockInfoRepo,
  ) {}

  getBlockChainInfo(): BlockChainInfo {
    return {
      version: "abc",
      supply: 1,
      transactionAverage: 1,
      hashRate: "abc",
    };
  }
}
