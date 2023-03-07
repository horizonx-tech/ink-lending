import {
  PriceOracleChanged,
  RateStrategyChanged,
} from './../types/event-types/registry';
import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import RateStrategy_factory from '../types/constructors/rate_strategies';
import Token_factory from '../types/constructors/psp22_token';
import Registry from '../types/contracts/registry';
import AssetPool from '../types/contracts/asset_pool';
import Factory from '../types/contracts/factory';
import RateStrategy from '../types/contracts/rate_strategies';
import PSP22Token from '../types/contracts/psp22_token';
import { Hash } from 'types-arguments/factory';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit } from './testHelpers';
import { FactoryChanged } from 'event-types/registry';
import {
  deployAssetPool,
  deployFactory,
  deployPSP22Token,
  deployRateStrategy,
  deployRegistry,
  deploySharesToken,
} from './testContractsHelpers';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Registry spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;

  let registry: Registry;
  let assetPool: AssetPool;
  let factory: Factory;
  let rateStrategy: RateStrategy;
  let token: PSP22Token;

  let assetPoolHash: Hash;
  let sharesHash: Hash;

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer } = globalThis.setup);

    registry = await deployRegistry({
      api,
      signer: deployer,
      args: [zeroAddress, deployer.address],
    });

    assetPool = await deployAssetPool({
      api,
      signer: deployer,
      args: [zeroAddress, zeroAddress, zeroAddress, zeroAddress],
    });
    assetPoolHash = assetPool.abi.info.source.wasmHash.toHex();

    token = await deployPSP22Token({
      api,
      signer: deployer,
      args: [1_000, ['Dai Stablecoin'], ['DAI'], 18],
    });

    const shares = await deploySharesToken({
      api,
      signer: deployer,
      args: [token.address, [], [], 18],
    });
    sharesHash = shares.abi.info.source.wasmHash.toHex();

    factory = await deployFactory({
      api,
      signer: deployer,
      args: [registry.address, assetPoolHash, sharesHash, deployer.address],
    });

    rateStrategy = await deployRateStrategy({
      api,
      signer: deployer,
      args: [],
    });
  };

  beforeAll(async () => {
    await setup();
  });

  describe('factory', () => {
    it('set factory', async () => {
      const res = await registry.tx.setFactory(factory.address);

      expect(res.events).toHaveLength(1);
      expectToEmit<FactoryChanged>(res.events[0], 'FactoryChanged', {
        factory: factory.address,
      });

      const {
        value: { ok: registryAddress },
      } = await factory.query.registry();
      expect(registryAddress).toBe(registry.address);
    });

    it('register pool', async () => {
      const {
        value: {
          ok: { ok: expectedPoolAddress },
        },
      } = await factory.query.create(token.address, []);
      expect(expectedPoolAddress).not.toBe(zeroAddress);

      let eventEmitted = false;
      registry.events.subscribeOnPoolRegisteredEvent((event) => {
        expect(event).toEqual({
          pool: expectedPoolAddress,
          asset: token.address,
        });
        eventEmitted = true;
      });
      await factory.tx.create(token.address, []);

      expect(eventEmitted).toBeTruthy();

      const {
        value: { ok: poolAddress },
      } = await registry.query.pool(token.address);
      expect(poolAddress).toBe(expectedPoolAddress);
    });
  });

  describe('rate strategy', () => {
    it('set rate strategy without asset', async () => {
      const res = await registry.tx.setRateStrategy(rateStrategy.address, null);

      expect(res.events).toHaveLength(1);
      expectToEmit<RateStrategyChanged>(res.events[0], 'RateStrategyChanged', {
        strategy: rateStrategy.address,
        asset: null,
      });

      const {
        value: { ok: rateStrategyAddress },
      } = await registry.query.rateStrategy(token.address);
      expect(rateStrategyAddress).toBe(rateStrategy.address);
    });
    it('set rate strategy with asset', async () => {
      const anotherStrategy = new RateStrategy(
        (await new RateStrategy_factory(api, deployer).new()).address,
        deployer,
        api,
      ).address;

      const anotherToken = new PSP22Token(
        (
          await new Token_factory(api, deployer).new(1_000, null, null, 18)
        ).address,
        deployer,
        api,
      ).address;
      const res = await registry.tx.setRateStrategy(
        anotherStrategy,
        anotherToken,
      );

      expect(res.events).toHaveLength(1);
      expectToEmit<RateStrategyChanged>(res.events[0], 'RateStrategyChanged', {
        strategy: anotherStrategy,
        asset: anotherToken,
      });

      let {
        value: { ok: address },
      } = await registry.query.rateStrategy(anotherToken);
      expect(address).toBe(anotherStrategy);

      ({
        value: { ok: address },
      } = await registry.query.rateStrategy(token.address));
      expect(address).toBe(rateStrategy.address);
    });
  });
  it('set price oracle', async () => {
    const priceOracle = encodeAddress(
      '0x0000000000000000000000000000000000000000000000000000000000000001',
    );
    const res = await registry.tx.setPriceOracle(priceOracle);

    expect(res.events).toHaveLength(1);
    expectToEmit<PriceOracleChanged>(res.events[0], 'PriceOracleChanged', {
      priceOracle,
    });

    const {
      value: { ok: actual },
    } = await registry.query.priceOracle();
    expect(actual).toBe(priceOracle);
  });
});
