import { useEffect, useState } from 'react';
import { usePolkadotApi } from './usePolkadotApi';
import UIDataProvider from 'src/__generated__/types/contracts/ui_data_provider';
import { uiPoolDataProvider } from 'src/api';
import { useSigner } from './useSigner';

export const useUIPoolDataProvider = () => {
  const { api } = usePolkadotApi();
  const { signer } = useSigner();
  const [provider, setProvider] = useState<UIDataProvider>();

  useEffect(() => {
    if (!api || !signer) return;
    console.log(api, signer);
    setProvider(uiPoolDataProvider(api, signer));
  }, [api]);

  return { provider };
};
