import { expect } from '@jest/globals';
import SharesToken_factory from '../types/constructors/shares_token';
import Token_factory from '../types/constructors/psp22_token';
import Token from '../types/contracts/psp22_token';
import SharesToken from '../types/contracts/shares_token';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit, hexToUtf8 } from './testHelpers';
import { Transfer } from 'event-types/psp22_token';

describe('SharesToken spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let sharesFactory: SharesToken_factory;
  let tokenFactory: Token_factory;

  let token: Token;
  let shares: SharesToken;

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);

    tokenFactory = new Token_factory(api, deployer);
    token = new Token(
      (
        await tokenFactory.new(
          1_000,
          'Dai Stablecoin' as unknown as string[],
          'DAI' as unknown as string[],
          18,
        )
      ).address,
      deployer,
      api,
    );
    sharesFactory = new SharesToken_factory(api, deployer);
    shares = new SharesToken(
      (await sharesFactory.new(token.address, null, null, 18)).address,
      deployer,
      api,
    );
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
        await sharesFactory.new(
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
