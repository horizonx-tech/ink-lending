import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import Registry from '../types/contracts/registry';
import AssetPool from '../types/contracts/asset_pool';
import Factory from '../types/contracts/factory';
import RateStrategy from '../types/contracts/rate_strategies';
import RiskStrategy from '../types/contracts/risk_strategies';
import PSP22Token from '../types/contracts/psp22_token';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { deployAssetPool, deployFactory, deployPSP22Token, deployRateStrategy, deployRegistry, deployRiskStrategy, deploySharesToken } from './testContractsHelpers';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Lending spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registry: Registry;
  let assetPool: AssetPool;
  let factory: Factory;
  let rateStrategy: RateStrategy;
  let riskStrategy: RiskStrategy;
  let token: PSP22Token;

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);
    registry = await deployRegistry({
      api,
      signer: deployer,
      args: [zeroAddress, deployer.address]
    })

    assetPool = await deployAssetPool({
      api,
      signer: deployer,
      args: [zeroAddress, zeroAddress, zeroAddress, zeroAddress]
    });
    const assetPoolHash = assetPool.abi.info.source.wasmHash.toHex();

    token = await deployPSP22Token({
      api,
      signer: deployer,
      args: [1_000, ['Dai Stablecoin'], ['DAI'], 18]
    })

    const shares = await deploySharesToken({
      api,
      signer: deployer,
      args: [token.address, [], [], 18]
    })
    const sharesHash = shares.abi.info.source.wasmHash.toHex();

    factory = await deployFactory({
      api,
      signer: deployer,
      args: [registry.address, assetPoolHash, sharesHash, deployer.address]
    })

    rateStrategy = await deployRateStrategy({
      api,
      signer: deployer,
      args: []
    })

    riskStrategy = await deployRiskStrategy({
      api,
      signer: deployer,
      args: [null]
    })

    // initialize
    await registry.tx.setRateStrategy(rateStrategy.address, null);
    await registry.tx.setRiskStrategy(riskStrategy.address, null);
    await registry.tx.setFactory(factory.address, null);
  };

  it('initialized', async () => {
    await setup();

    // registry
    const {
      value: { ok: factoryAddress },
    } = await registry.query.factory();
    expect(factoryAddress).toBe(factory.address);

    const {
      value: { ok: rateStrategyAddress },
    } = await registry.query.rateStrategy(token.address);
    expect(rateStrategyAddress).toBe(rateStrategy.address);

    const {
      value: { ok: riskStrategyAddress },
    } = await registry.query.riskStrategy(token.address);
    expect(riskStrategyAddress).toBe(riskStrategy.address);

    // factory
    const {
      value: { ok: registryAddress },
    } = await factory.query.registry();
    expect(registryAddress).toBe(registry.address);
  });

  it('create pool', async () => {
    const {
      value: {
        ok: { ok: expectedPoolAddress },
      },
    } = await factory.query.create(token.address, []);
    expect(expectedPoolAddress).not.toBe(zeroAddress);

    await factory.tx.create(token.address, []);
    const {
      value: { ok: poolAddress },
    } = await registry.query.pool(token.address);
    expect(poolAddress).toBe(expectedPoolAddress);
  });
});
