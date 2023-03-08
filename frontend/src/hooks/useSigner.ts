import Keyring from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import { waitReady } from '@polkadot/wasm-crypto';
import { useEffect, useState } from 'react';
import { env } from 'src/env';

// TODO context
const keyring = new Keyring({ type: env.keyringType });
export const useSigner = () => {
  const [signer, setSigner] = useState<KeyringPair>();

  useEffect(() => {
    waitReady().then(() => {
      setSigner(keyring.addFromUri('//Alice'));
    });
  }, []);

  return { signer };
};
