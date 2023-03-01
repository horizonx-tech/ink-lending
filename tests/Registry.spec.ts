import { RateStrategyChanged } from './../types/event-types/registry';
import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import Registry_factory from '../types/constructors/registry';
import AssetPool_factory from '../types/constructors/asset_pool';
import Factory_factory from '../types/constructors/factory';
import SharesToken_factory from '../types/constructors/shares_token';
import RateStrategy_factory from '../types/constructors/rate_strategies';
import RiskStrategy_factory from '../types/constructors/risk_strategies';
import Token_factory from '../types/constructors/psp22_token';
import Registry from '../types/contracts/registry';
import AssetPool from '../types/contracts/asset_pool';
import Factory from '../types/contracts/factory';
import RateStrategy from '../types/contracts/rate_strategies';
import RiskStrategy from '../types/contracts/risk_strategies';
import Token from '../types/contracts/psp22_token';
import SharesToken from '../types/contracts/shares_token';
import { Hash } from 'types-arguments/factory';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { WeightV2 } from '@polkadot/types/interfaces';
import { expectToEmit } from './testHelpers';
import { FactoryChanged, PoolRegistered } from 'event-types/registry';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Registry spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registryFactory: Registry_factory;
  let assetPoolFactory: AssetPool_factory;
  let factoryFactory: Factory_factory;
  let rateStrategyFactory: RateStrategy_factory;
  let riskStrategyFactory: RiskStrategy_factory;
  let sharesFactory: SharesToken_factory;
  let tokenFactory: Token_factory;

  let registry: Registry;
  let assetPool: AssetPool;
  let factory: Factory;
  let rateStrategy: RateStrategy;
  let riskStrategy: RiskStrategy;
  let shares: SharesToken;
  let token: Token;

  let assetPoolHash: Hash;
  let sharesHash: Hash;

  let gasRequired: WeightV2;

  async function setup(): Promise<void> {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);
    registryFactory = new Registry_factory(api, deployer);
    registry = new Registry(
      (await registryFactory.new()).address,
      deployer,
      api,
    );

    assetPoolFactory = new AssetPool_factory(api, deployer);
    assetPool = new AssetPool(
      (
        await assetPoolFactory.new(
          zeroAddress,
          zeroAddress,
          zeroAddress,
          zeroAddress,
        )
      ).address,
      deployer,
      api,
    );
    assetPoolHash = assetPool.abi.info.source.wasmHash.toHex();

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
      (
        await factoryFactory.new(registry.address, assetPoolHash, sharesHash)
      ).address,
      deployer,
      api,
    );

    rateStrategyFactory = new RateStrategy_factory(api, deployer);
    rateStrategy = new RateStrategy(
      (await rateStrategyFactory.new()).address,
      deployer,
      api,
    );
    riskStrategyFactory = new RiskStrategy_factory(api, deployer);
    riskStrategy = new RiskStrategy(
      (await riskStrategyFactory.new()).address,
      deployer,
      api,
    );
  }

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

      // await registry.tx.setRateStrategy(rateStrategy.address, null);
      // await registry.tx.setRiskStrategy(riskStrategy.address, null);
      // const {
      //   value: { ok: rateStrategyAddress },
      // } = await registry.query.rateStrategy(token.address);
      // expect(rateStrategyAddress).toBe(rateStrategy.address);

      // const {
      //   value: { ok: riskStrategyAddress },
      // } = await registry.query.riskStrategy(token.address);
      // expect(riskStrategyAddress).toBe(riskStrategy.address);

      // factory
    });

    it('register pool', async () => {
      const {
        gasRequired,
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
      await factory.tx.create(token.address, [], {
        gasLimit: gasRequired,
      });

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
        (await rateStrategyFactory.new()).address,
        deployer,
        api,
      ).address;

      const anotherToken = new Token(
        (await tokenFactory.new(1_000, null, null, 18)).address,
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
});
