import { ApiPromise, WsProvider } from '@polkadot/api';
import { useEffect, useState } from 'react';
import { env } from 'src/env';

// TODO context
export const usePolkadotApi = (endpoint = env.apiEndpoint) => {
  const [api, setApi] = useState<ApiPromise>();

  useEffect(() => {
    const wsProvider = new WsProvider(endpoint);
    ApiPromise.create({
      provider: wsProvider,
      throwOnConnect: true,
    }).then(setApi);
  }, []);

  return { api };
};
