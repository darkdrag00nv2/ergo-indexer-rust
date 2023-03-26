export class HexString {
  constructor(public value: string) {
    // TODO: validate
  }
}

export type BlockId = HexString;

export class Address {
  constructor(public value: string) {
    // TODO: validate
  }
}

export class MonetarySettings {
}

export class ProtocolSettings {
  constructor(
    private networkPrefix: string,
    private genesisAddress: Address,
    private monetary: MonetarySettings,
  ) {}
}
