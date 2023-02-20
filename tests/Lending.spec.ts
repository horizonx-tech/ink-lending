import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import BN from 'bn.js';
import Registry_factory from '../types/constructors/registry';
import Factory_factory from '../types/constructors/factory';
import SharesToken_factory from '../types/constructors/shares_token';
import Token_factory from '../types/constructors/psp22_token';
import Registry from '../types/contracts/registry';
import Factory from '../types/contracts/factory';
import Token from '../types/contracts/psp22_token';
import SharesToken from '../types/contracts/shares_token';
import { AccountId, Hash } from 'types-arguments/factory';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { emit, revertedWith } from './testHelpers';
import type { WeightV2 } from '@polkadot/types/interfaces';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);
const MINIMUM_LIQUIDITY = 1000;

describe('Dex spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registryFactory: Registry_factory;
  let factoryFactory: Factory_factory;
  let sharesFactory: SharesToken_factory;
  let tokenFactory: Token_factory;

  let sharesHash: Hash;
  let registry: Registry;
  let factory: Factory;
  let shares: SharesToken;
  let token: Token;

  let gasRequired: WeightV2;

  async function setup(): Promise<void> {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);
    registryFactory = new Registry_factory(api, deployer);
    registry = new Registry(
      (await registryFactory.new()).address,
      deployer,
      api,
    );

    tokenFactory = new Token_factory(api, deployer);
    token = new Token(
      (await tokenFactory.new(1_000, ['Dai Stablecoin'], ['DAI'], 18)).address,
      deployer,
      api,
    );
    sharesFactory = new SharesToken_factory(api, deployer);
    const shares = new SharesToken(
      (await sharesFactory.new(token.address, [], [], 18)).address,
      deployer,
      api,
    );
    sharesHash = shares.abi.info.source.wasmHash.toHex();

    factoryFactory = new Factory_factory(api, deployer);
    factory = new Factory(
      (await factoryFactory.new(registry.address, sharesHash)).address,
      deployer,
      api,
    );
  }

  it('create pool', async () => {
    await setup();
    console.log(token.address);
    const {
      value: { ok: registryAddress },
    } = await factory.query.registry();
    expect(registryAddress).toBe(registry.address);
    let {
      gasRequired,
      value: {
        ok: { ok: expectedAddress },
      },
    } = await factory.query.create(token.address, []);
    console.log(expectedAddress);
    expect(expectedAddress).not.toBe(zeroAddress);
    const result = await factory.tx.create(token.address, [], {
      gasLimit: gasRequired,
    });
    // expect(result).not.toBe(expectedAddress);
  });
});
