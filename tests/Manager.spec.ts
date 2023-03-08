import { encodeAddress } from '@polkadot/keyring';
import type { KeyringPair } from '@polkadot/keyring/types';
import Registry from '../types/contracts/registry';
import Manager from '../types/contracts/manager';
import { deployManager, deployRegistry } from './testContractsHelpers';

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

    // initialize
    await registry.tx.setManager(manager.address);

    return {
      api,
      deployer,
      registry,
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
})