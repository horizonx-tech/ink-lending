import { Abi } from '@polkadot/api-contract';
import { encodeAddress } from '@polkadot/keyring';
import type { KeyringPair } from '@polkadot/keyring/types';
import Registry from '../types/contracts/registry';
import Factory from '../types/contracts/factory';
import Manager from '../types/contracts/manager';
import ASSET_POOL_ABI from '../artifacts/asset_pool.json';
import SHARES_TOKEN_ABI from '../artifacts/shares_token.json';
import { deployAssetPool, deployFactory, deployManager, deployRegistry, deploySharesToken } from './testContractsHelpers';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);
const Roles = {
  DefaultAdminRole: 0,
  PoolAdmin: 2401180619,
}

describe("Manager spec", () => {
  const setup = async () => {
    const { api, alice: deployer } = globalThis.setup;

    const registry = await deployRegistry({
      api,
      signer: deployer,
      args: [deployer.address],
    });

    const manager = await deployManager({
      api,
      signer: deployer,
      args: [registry.address],
    });

    const assetPool = await deployAssetPool({
      api,
      signer: deployer,
      args: [zeroAddress, zeroAddress, zeroAddress, zeroAddress],
    });
    const assetPoolHash = assetPool.abi.info.source.wasmHash.toHex();
    // const assetPoolHash = new Abi(ASSET_POOL_ABI).info.source.wasmHash.toHex();
    const shares = await deploySharesToken({
      api,
      signer: deployer,
      args: [zeroAddress, [], [], 18],
    });
    const sharesHash = shares.abi.info.source.wasmHash.toHex();
    // const sharesHash = new Abi(SHARES_TOKEN_ABI).info.source.wasmHash.toHex();
    const factory = await deployFactory({
      api,
      signer: deployer,
      args: [registry.address, assetPoolHash, sharesHash],
    });

    // initialize
    await registry.tx.setFactory(factory.address);
    await registry.tx.setManager(manager.address);

    return {
      api,
      deployer,
      registry,
      factory,
      manager
    }
  }

  it("initialized", async () => {
    const { deployer, registry, manager } = await setup()

    expect((await manager.query.registry()).value.ok).toBe(registry.address);
    expect((await manager.query.hasRole(Roles.DefaultAdminRole, deployer.address)).value.ok).toBeTruthy;
    expect((await manager.query.hasRole(Roles.PoolAdmin, deployer.address)).value.ok).toBeFalsy;
  })

  describe("call Registry", () => {
    let deployer: KeyringPair;
    let registry: Registry;
    let manager: Manager;

    const strategy = encodeAddress('0x0000000000000000000000000000000000000000000000000000000000000001');

    beforeAll(async () => {
      ({ deployer, registry, manager } = await setup());
    });

    it("preparations", async () => {
      // for manager
      await manager.tx.grantRole(Roles.PoolAdmin, deployer.address);
      expect((await manager.query.hasRole(Roles.PoolAdmin, deployer.address)).value.ok).toBeTruthy;

      // for registry
      expect((await registry.query.manager()).value.ok).toBe(manager.address);
    })

    it(".updateRateStrategy", async () => {
      expect((await registry.query.defaultRateStrategy()).value.ok).toBe(zeroAddress);

      await manager.tx.updateRateStrategy(strategy, null);
      expect((await registry.query.defaultRateStrategy()).value.ok).toBe(strategy);
    })

    it(".updateRateStrategy", async () => {
      expect((await registry.query.defaultRiskStrategy()).value.ok).toBe(zeroAddress);

      await manager.tx.updateRiskStrategy(strategy, null);
      expect((await registry.query.defaultRiskStrategy()).value.ok).toBe(strategy);
    })
  })

  describe("call Factory", () => {
    let deployer: KeyringPair;
    let registry: Registry;
    let factory: Factory;
    let manager: Manager;

    beforeAll(async () => {
      ({ deployer, registry, factory, manager } = await setup());
    });

    it("preparations", async () => {
      // for manager
      await manager.tx.grantRole(Roles.PoolAdmin, deployer.address);
      expect((await manager.query.hasRole(Roles.PoolAdmin, deployer.address)).value.ok).toBeTruthy;

      // for registry
      expect((await registry.query.manager()).value.ok).toBe(manager.address);
      expect((await registry.query.factory()).value.ok).toBe(factory.address);
    })

    it(".create_pool", async () => {
      const token = encodeAddress('0x0000000000000000000000000000000000000000000000000000000000000001');
      expect((await registry.query.pool(token)).value.ok).toBeNull;

      const {
        value: {
          ok: { ok: expectedPoolAddress },
        },
      } = await manager.query.createPool(token, []);
      await manager.withSigner(deployer).tx.createPool(token, []);
      expect((await registry.query.pool(token)).value.ok).toBe(expectedPoolAddress);
    })
  })
})