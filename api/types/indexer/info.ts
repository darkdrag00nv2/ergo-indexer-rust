export interface BlockChainInfo {
  version: string;
  supply: number;
  transactionAverage: number;
  hashRate: number;
}

export function getDefaultBlockChainInfo(): BlockChainInfo {
  return {
    version: "0.0.0",
    supply: 0,
    transactionAverage: 0,
    hashRate: 0,
  };
}
