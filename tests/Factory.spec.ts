import { ApiPromise } from '@polkadot/api';
import { Abi } from '@polkadot/api-contract';
import { encodeAddress } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import { deployAssetPool, deployFactory, deployPSP22Token, deployRegistry, deploySharesToken } from './testContractsHelpers';
import Registry from '../types/contracts/registry';
import Factory from '../types/contracts/factory';
import PSP22Token from '../types/contracts/psp22_token';
import ASSET_POOL_ABI from '../artifacts/asset_pool.json';
import SHARES_TOKEN_ABI from '../artifacts/shares_token.json';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Factory spec', () => {
  const setup = async () => {
    const { api, alice: deployer } = globalThis.setup;

    const registry = await deployRegistry({
      api,
      signer: deployer,
      args: [deployer.address],
    });

    const assetPool = await deployAssetPool({
      api,
      signer: deployer,
      args: [zeroAddress, zeroAddress, zeroAddress, zeroAddress],
    });
    const assetPoolHash = assetPool.abi.info.source.wasmHash.toHex();
    const token = await deployPSP22Token({
      api,
      signer: deployer,
      args: [1_000, ['Dai Stablecoin'], ['DAI'], 18],
    });
    const shares = await deploySharesToken({
      api,
      signer: deployer,
      args: [token.address, [], [], 18],
    });
    const sharesHash = shares.abi.info.source.wasmHash.toHex();
    // const assetPoolHash = new Abi(ASSET_POOL_ABI).info.source.wasmHash.toHex();
    // const sharesHash = new Abi(SHARES_TOKEN_ABI).info.source.wasmHash.toHex();
    const factory = await deployFactory({
      api,
      signer: deployer,
      args: [registry.address, assetPoolHash, sharesHash],
    });

    return {
      api,
      deployer: (deployer as KeyringPair),
      registry,
      factory,
    }
  }

  it("initialized", async () => {
    const { registry, factory } = await setup()

    expect((await factory.query.registry()).value.ok).toBe(registry.address);
  })

  describe(".create", () => {
    let api: ApiPromise;
    let deployer: KeyringPair;
    let registry: Registry;
    let factory: Factory;
    let token: PSP22Token;

    beforeAll(async () => {
      ({ api, deployer, registry, factory } = await setup());

      token = await deployPSP22Token({
        api,
        signer: deployer,
        args: [1_000, ['Dai Stablecoin'], ['DAI'], 18],
      });
    });

    it("preparations", async () => {
      // for registry
      await registry.tx.setFactory(factory.address);
      expect((await registry.query.manager()).value.ok).toBe(deployer.address);
      expect((await registry.query.factory()).value.ok).toBe(factory.address);
    })

    it("execute", async () => {
      const {
        value: {
          ok: { ok: expectedPoolAddress }
        },
      } = await factory.query.create(token.address, []); // dry-run

      await factory.tx.create(token.address, []);
      expect((await registry.query.pool(token.address)).value.ok).toBe(expectedPoolAddress);

      // event
    })

    it.skip("check generated asset_pool", () => {})
  })
})