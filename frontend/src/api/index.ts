import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { env } from 'src/env';
import UIDataProvider from 'src/__generated__/types/contracts/ui_data_provider';

export const uiPoolDataProvider = (api: ApiPromise, signer: KeyringPair) =>
  new UIDataProvider(env.addresses.uiDataProvider, signer, api);
