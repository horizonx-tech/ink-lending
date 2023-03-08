import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import Registry_factory from '../types/constructors/registry';
import DummyPool_factory from '../types/constructors/dummy_pool';
import DummyOracle_factory from '../types/constructors/dummy_oracle';
import Factory_factory from '../types/constructors/factory';
import SharesToken_factory from '../types/constructors/shares_token';
import RateStrategy_factory from '../types/constructors/rate_strategies';
import RiskStrategy_factory from '../types/constructors/risk_strategies';
import UIDataProvider_factory from '../types/constructors/ui_data_provider';
import Token_factory from '../types/constructors/psp22_token';
import Registry from '../types/contracts/registry';
import DummyPool from '../types/contracts/dummy_pool';
import DummyOracle from '../types/contracts/dummy_oracle';
import Factory from '../types/contracts/factory';
import RateStrategy from '../types/contracts/rate_strategies';
import RiskStrategy from '../types/contracts/risk_strategies';
import UIDataProvider from '../types/contracts/ui_data_provider';
import Token from '../types/contracts/psp22_token';
import SharesToken from '../types/contracts/shares_token';
import { Hash } from 'types-arguments/factory';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { WeightV2 } from '@polkadot/types/interfaces';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Lending spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registryFactory: Registry_factory;
  let dummyPoolFactory: DummyPool_factory;
  let dummyOracleFactory: DummyOracle_factory;
  let factoryFactory: Factory_factory;
  let rateStrategyFactory: RateStrategy_factory;
  let riskStrategyFactory: RiskStrategy_factory;
  let sharesFactory: SharesToken_factory;
  let tokenFactory: Token_factory;

  let registry: Registry;
  let dummyPool: DummyPool;
  let dummyOracle: DummyOracle;
  let factory: Factory;
  let rateStrategy: RateStrategy;
  let riskStrategy: RiskStrategy;
  let token: Token;

  let uiDataProvider: UIDataProvider;

  let assetPoolHash: Hash;
  let sharesHash: Hash;

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);
    registryFactory = new Registry_factory(api, deployer);
    registry = new Registry(
      (await registryFactory.new(null)).address,
      deployer,
      api,
    );

    const uiDataProviderFactory = new UIDataProvider_factory(api, deployer);
    uiDataProvider = new UIDataProvider(
      (await uiDataProviderFactory.new(registry.address)).address,
      deployer,
      api,
    );

    dummyPoolFactory = new DummyPool_factory(api, deployer);
    dummyPool = new DummyPool(
      (
        await dummyPoolFactory.new(
          zeroAddress,
          zeroAddress,
          zeroAddress,
          zeroAddress,
        )
      ).address,
      deployer,
      api,
    );
    assetPoolHash = dummyPool.abi.info.source.wasmHash.toHex();

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
      (await riskStrategyFactory.new(null)).address,
      deployer,
      api,
    );
    dummyOracleFactory = new DummyOracle_factory(api, deployer);
    dummyOracle = new DummyOracle(
      (await dummyOracleFactory.new()).address,
      deployer,
      api,
    );
    await registry.tx.setRateStrategy(rateStrategy.address, null);
    await registry.tx.setRiskStrategy(riskStrategy.address, null);
    await registry.tx.setFactory(factory.address);
    await registry.tx.setPriceOracle(dummyOracle.address);
  };

  beforeAll(async () => {
    await setup();
  });
  describe('pool data', () => {
    const asset1 = encodeAddress(
      '0x0000000000000000000000000000000000000000000000000000000000000001',
    );
    const asset2 = encodeAddress(
      '0x0000000000000000000000000000000000000000000000000000000000000002',
    );
    beforeAll(async () => {
      await factory.tx.create(asset1, []);
      await factory.tx.create(asset2, []);
    });
    it('assets', async () => {
      const {
        value: { ok: assets },
      } = await uiDataProvider.query.assets();
      expect(assets).toHaveLength(2);
      expect(assets[0]).toBe(asset1);
      expect(assets[1]).toBe(asset2);
    });
    it('pools', async () => {
      const {
        value: { ok: pools },
      } = await uiDataProvider.query.pools(null);
      expect(pools).toHaveLength(2);
      expect(pools[0].asset).toBe(asset1);
      expect(pools[0].collateralToken).not.toBe(zeroAddress);
      expect(pools[0].debtToken).not.toBe(zeroAddress);
      expect(pools[0].liquidityIndex.toNumber()).toBe(0);
      expect(pools[0].liquidityRate.toNumber()).toBe(0);
      expect(pools[0].debtIndex.toNumber()).toBe(0);
      expect(pools[0].debtRate.toNumber()).toBe(0);
      expect(pools[0].lastUpdateTimestamp).toBe(0);
      expect(pools[1].asset).toBe(asset2);
    });
    it('market data', async () => {
      const {
        value: { ok: marketData },
      } = await uiDataProvider.query.marketData(null);
      expect(marketData).toHaveLength(2);
      // TODO test mapping
      expect(marketData[0].asset).toBe(asset1);
      expect(marketData[0].price.toNumber()).not.toBe(zeroAddress);
      expect(marketData[0].liquidityShare.toNumber()).not.toBe(zeroAddress);
      expect(marketData[0].liquidityIndex.toNumber()).toBe(0);
      expect(marketData[0].liquidityInterestRate.toNumber()).toBe(0);
      expect(marketData[0].debtInterestRate.toNumber()).toBe(0);
      expect(marketData[0].debtIndex.toNumber()).toBe(0);
      expect(marketData[0].debtInterestRate.toNumber()).toBe(0);
      expect(marketData[0].poolLastUpdateTimestamp).toBe(0);
      expect(marketData[0].loanToValue).toBe(0);
      expect(marketData[0].liquidationThreshold).toBe(0);
      expect(marketData[1].asset).toBe(asset2);
    });
  });
});
