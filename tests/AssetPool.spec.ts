import { Abi } from '@polkadot/api-contract';
import { encodeAddress } from '@polkadot/keyring';
import { deployAssetPool, deployRegistry } from './testContractsHelpers';
import ASSET_POOL_ABI from '../artifacts/asset_pool.json';
import SHARES_TOKEN_ABI from '../artifacts/shares_token.json';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('AssetPool spec', () => {
  describe('only AssetPool', () => {
    const setup = async () => {
      const { api, alice: deployer, bob: notManager } = globalThis.setup;

      const registry = await deployRegistry({
        api,
        signer: deployer,
        args: [deployer.address],
      });

      const assetPool = await deployAssetPool({
        api,
        signer: deployer,
        args: [
          registry.address,
          zeroAddress,
          zeroAddress,
          zeroAddress,
          null,
          null,
        ],
      });

      return { deployer, registry, assetPool, notManager };
    };

    describe('.set_deposit_paused', () => {
      it('success', async () => {
        const { deployer, assetPool } = await setup();

        expect((await assetPool.query.depositPaused()).value.ok).toBeFalsy;
        await assetPool.withSigner(deployer).tx.setDepositPaused(true);
        expect((await assetPool.query.depositPaused()).value.ok).toBeTruthy;
        await assetPool.withSigner(deployer).tx.setDepositPaused(false);
        expect((await assetPool.query.depositPaused()).value.ok).toBeFalsy;
      });

      it.skip('fail with notManager', async () => {});
    });
    describe('.set_borrow_paused', () => {
      it('success', async () => {
        const { deployer, assetPool } = await setup();

        expect((await assetPool.query.borrowPaused()).value.ok).toBeFalsy;
        await assetPool.withSigner(deployer).tx.setBorrowPaused(true);
        expect((await assetPool.query.borrowPaused()).value.ok).toBeTruthy;
        await assetPool.withSigner(deployer).tx.setBorrowPaused(false);
        expect((await assetPool.query.borrowPaused()).value.ok).toBeFalsy;
      });

      it.skip('fail with notManager', async () => {});
    });
  });
});
