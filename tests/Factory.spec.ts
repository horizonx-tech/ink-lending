import { ApiPromise } from '@polkadot/api';
import { Abi } from '@polkadot/api-contract';
import { KeyringPair } from '@polkadot/keyring/types';
import {
  deployFactory,
  deployPSP22Token,
  deployRegistry,
} from './testContractsHelpers';
import AssetPool from '../types/contracts/asset_pool';
import SharesToken from '../types/contracts/shares_token';
import Registry from '../types/contracts/registry';
import Factory from '../types/contracts/factory';
import PSP22Token from '../types/contracts/psp22_token';
import ASSET_POOL_ABI from '../artifacts/asset_pool.json';
import SHARES_TOKEN_ABI from '../artifacts/shares_token.json';
import { hexToUtf8 } from './testHelpers';

describe('Factory spec', () => {
  const setup = async () => {
    const { api, alice: deployer } = globalThis.setup;

    const registry = await deployRegistry({
      api,
      signer: deployer,
      args: [deployer.address],
    });

    const assetPoolHash = new Abi(ASSET_POOL_ABI).info.source.wasmHash.toHex();
    const sharesHash = new Abi(SHARES_TOKEN_ABI).info.source.wasmHash.toHex();
    const factory = await deployFactory({
      api,
      signer: deployer,
      args: [registry.address, assetPoolHash, sharesHash],
    });

    return {
      api,
      deployer: deployer as KeyringPair,
      registry,
      factory,
    };
  };

  it('initialized', async () => {
    const { registry, factory } = await setup();

    expect((await factory.query.registry()).value.ok).toBe(registry.address);
  });

  describe('.create', () => {
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
        args: [
          1_000,
          'Dai Stablecoin' as unknown as string[],
          'DAI' as unknown as string[],
          18
        ],
      });
    });

    it('preparations', async () => {
      // for registry
      await registry.tx.setFactory(factory.address);
      expect((await registry.query.manager()).value.ok).toBe(deployer.address);
      expect((await registry.query.factory()).value.ok).toBe(factory.address);
    });

    it('execute', async () => {
      const {
        value: {
          ok: { ok: expectedPoolAddress },
        },
      } = await factory.query.create(token.address, []); // dry-run

      await factory.tx.create(token.address, []);
      expect((await registry.query.pool(token.address)).value.ok).toBe(
        expectedPoolAddress,
      );
    });

    it('check generated asset_pool', async () => {
      const assetPoolAddr = (await registry.query.pool(token.address)).value.ok;
      const assetPool = new AssetPool(assetPoolAddr as string, deployer, api);
      const collateralTokenAddr = (await assetPool.query.collateralToken())
        .value.ok;
      const debtTokenAddr = (await assetPool.query.debtToken()).value.ok;
      const collateralToken = new SharesToken(
        collateralTokenAddr as string,
        deployer,
        api,
      );
      const debtToken = new SharesToken(debtTokenAddr as string, deployer, api);

      expect((await collateralToken.query.asset()).value.ok).toBe(
        token.address,
      );
      expect((await collateralToken.query.owner()).value.ok).toBe(
        assetPoolAddr,
      );
      expect(
        hexToUtf8((await collateralToken.query.tokenName()).value.ok),
      ).toBe('collateral Dai Stablecoin');
      expect(
        hexToUtf8((await collateralToken.query.tokenSymbol()).value.ok),
      ).toBe('cDAI');

      expect((await debtToken.query.asset()).value.ok).toBe(token.address);
      expect((await debtToken.query.owner()).value.ok).toBe(assetPoolAddr);
      expect(hexToUtf8((await debtToken.query.tokenName()).value.ok)).toBe(
        'debt Dai Stablecoin',
      );
      expect(hexToUtf8((await debtToken.query.tokenSymbol()).value.ok)).toBe(
        'vdDAI',
      );
    });
  });
});
