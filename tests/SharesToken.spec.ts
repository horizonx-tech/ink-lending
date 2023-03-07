import { expect } from '@jest/globals';
import SharesToken_factory from '../types/constructors/shares_token';
import SharesToken from '../types/contracts/shares_token';
import PSP22Token from '../types/contracts/psp22_token';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit, hexToUtf8 } from './testHelpers';
import { Transfer } from 'event-types/psp22_token';
import { deployPSP22Token, deploySharesToken } from './testContractsHelpers';

describe('SharesToken spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let shares: SharesToken;
  let token: PSP22Token;

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);

    token = await deployPSP22Token({
      api,
      signer: deployer,
      args: [1_000, ['Dai Stablecoin'], ['DAI'], 18]
    });

    shares = await deploySharesToken({
      api,
      signer: deployer,
      args: [token.address, null, null, 18]
    });
  };

  beforeAll(async () => {
    await setup();
  });

  it('constructor', async () => {
    const expectedName = 'SharesToken';
    const expectedSymbol = 'SHT';
    const expectedDecimals = 18;
    const expectedAsset = token.address;

    const instance = new SharesToken(
      (
        await (new SharesToken_factory(api, deployer)).new(
          expectedAsset,
          expectedName as unknown as string[],
          expectedSymbol as unknown as string[],
          expectedDecimals,
        )
      ).address,
      deployer,
      api,
    );

    expect(hexToUtf8((await instance.query.tokenName()).value.ok)).toBe(
      expectedName,
    );
    expect(hexToUtf8((await instance.query.tokenSymbol()).value.ok)).toBe(
      expectedSymbol,
    );
    expect((await instance.query.tokenDecimals()).value.ok).toBe(
      expectedDecimals,
    );
    expect((await instance.query.owner()).value.ok).toBe(deployer.address);
  });

  describe('event emission', () => {
    it('on mint', async () => {
      const to = wallet.address;
      const value = 100;
      const res = await shares.tx.mint(to, value);

      expect(res.events).toHaveLength(1);
      expectToEmit<Transfer>(res.events[0], 'Transfer', {
        from: null,
        to,
        value,
      });
    });
  });
});
