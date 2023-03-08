import {
  PriceOracleChanged,
  RateStrategyChanged,
} from './../types/event-types/registry';
import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import RateStrategy_factory from '../types/constructors/rate_strategies';
import Token_factory from '../types/constructors/psp22_token';
import Registry from '../types/contracts/registry';
import Factory from '../types/contracts/factory';
import RateStrategy from '../types/contracts/rate_strategies';
import PSP22Token from '../types/contracts/psp22_token';
import ASSET_POOL_ABI from '../artifacts/asset_pool.json';
import SHARES_TOKEN_ABI from '../artifacts/shares_token.json';
import { Hash } from 'types-arguments/factory';
import { ApiPromise } from '@polkadot/api';
import { Abi } from '@polkadot/api-contract';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit } from './testHelpers';
import { FactoryChanged } from 'event-types/registry';
import {
  deployFactory,
  deployPSP22Token,
  deployRateStrategy,
  deployRegistry,
} from './testContractsHelpers';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Registry spec', () => {
  const setup = async () => {
    const { api, alice: deployer } = globalThis.setup;

    const registry = await deployRegistry({
      api,
      signer: deployer,
      args: [deployer.address],
    });

    return {
      api,
      deployer: (deployer as KeyringPair),
      registry,
    }
  }

  it('initialized', async () => {
    const { deployer, registry } = await setup();

    expect((await registry.query.manager()).value.ok).toBe(deployer.address);
    expect((await registry.query.factory()).value.ok).toBe(zeroAddress);
    expect((await registry.query.priceOracle()).value.ok).toBe(zeroAddress);
    expect((await registry.query.assetsCount()).value.ok).toBe(0);
  });

  describe('factory', () => {
    let api: ApiPromise;
    let deployer: KeyringPair;

    let registry: Registry;
    let factory: Factory;
    let token: PSP22Token;

    beforeAll(async () => {
      ({ api, deployer, registry } = await setup());
      token = await deployPSP22Token({
        api,
        signer: deployer,
        args: [1_000, ['Dai Stablecoin'], ['DAI'], 18],
      });

      const assetPoolHash = new Abi(ASSET_POOL_ABI).info.source.wasmHash.toHex();
      const sharesHash = new Abi(SHARES_TOKEN_ABI).info.source.wasmHash.toHex();
      factory = await deployFactory({
        api,
        signer: deployer,
        args: [registry.address, assetPoolHash, sharesHash],
      });
    });

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
    let api: ApiPromise;
    let deployer: KeyringPair;

    let registry: Registry;
    let rateStrategy: RateStrategy;
    let token: PSP22Token;

    beforeAll(async () => {
      ({ api, deployer, registry } = await setup());
      rateStrategy = await deployRateStrategy({
        api,
        signer: deployer,
        args: [],
      });
      token = await deployPSP22Token({
        api,
        signer: deployer,
        args: [1_000, ['Dai Stablecoin'], ['DAI'], 18],
      });
    });

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
    const { registry } = await setup();

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
