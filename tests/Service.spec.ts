import { RateStrategyChanged } from '../types/event-types/registry';
import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import Registry_factory from '../types/constructors/registry';
import DummyPool_factory from '../types/constructors/dummy_pool';
import Service_factory from '../types/constructors/service';
import Registry from '../types/contracts/registry';
import DummyPool from '../types/contracts/dummy_pool';
import Service from '../types/contracts/service';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit } from './testHelpers';
import {
  Borrowed,
  Deposited,
  Repaid,
  Withdrew,
} from 'event-types/service';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Service spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registryFactory: Registry_factory;
  let dummyPoolFactory: DummyPool_factory;
  let serviceFactory: Service_factory;

  let registry: Registry;
  let dummyPool: DummyPool;
  let service: Service;

  const asset = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000001',
  );

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);
    registryFactory = new Registry_factory(api, deployer);
    registry = new Registry(
      (await registryFactory.new()).address,
      deployer,
      api,
    );

    dummyPoolFactory = new DummyPool_factory(api, deployer);
    dummyPool = new DummyPool(
      (
        await dummyPoolFactory.new(
          registry.address,
          asset,
          zeroAddress,
          zeroAddress,
        )
      ).address,
      deployer,
      api,
    );
    await registry.tx.registerPool(asset, dummyPool.address);

    serviceFactory = new Service_factory(api, deployer);
    service = new Service(
      (await serviceFactory.new(registry.address)).address,
      wallet,
      api,
    );
  };

  let signer;
  beforeAll(async () => {
    await setup();
    signer = wallet.address;
  });

  describe('event emission', () => {
    const amount = 1;

    const anotherAcconut = encodeAddress(
      '0x0000000000000000000000000000000000000000000000000000000000000002',
    );
    it('deposited', async () => {
      const res = await service.tx.deposit(asset, amount, null);
      expect(res.events).toHaveLength(1);
      expectToEmit<Deposited>(res.events[0], 'Deposited', {
        account: signer,
        amount,
        asset,
        from: signer,
      });
    });
    it('deposited to another account', async () => {
      const res = await service.tx.deposit(asset, amount, anotherAcconut);
      expect(res.events).toHaveLength(1);
      expectToEmit<Deposited>(res.events[0], 'Deposited', {
        account: anotherAcconut,
        amount,
        asset,
        from: signer,
      });
    });
    it('withdrew', async () => {
      const res = await service.tx.withdraw(asset, amount, null);
      expect(res.events).toHaveLength(1);
      expectToEmit<Withdrew>(res.events[0], 'Withdrew', {
        account: signer,
        amount,
        asset,
        to: signer,
      });
    });
    it('withdrew to another account', async () => {
      const res = await service.tx.withdraw(asset, amount, anotherAcconut);
      expect(res.events).toHaveLength(1);
      expectToEmit<Withdrew>(res.events[0], 'Withdrew', {
        account: signer,
        amount,
        asset,
        to: anotherAcconut,
      });
    });
    it('borrowed', async () => {
      const res = await service.tx.borrow(asset, amount, null);
      expect(res.events).toHaveLength(1);
      expectToEmit<Borrowed>(res.events[0], 'Borrowed', {
        account: signer,
        amount,
        asset,
        to: signer,
      });
    });
    it('borrowed to another account', async () => {
      const res = await service.tx.borrow(asset, amount, anotherAcconut);
      expect(res.events).toHaveLength(1);
      expectToEmit<Borrowed>(res.events[0], 'Borrowed', {
        account: signer,
        amount,
        asset,
        to: anotherAcconut,
      });
    });
    it('repaid', async () => {
      const res = await service.tx.repay(asset, amount, null);
      expect(res.events).toHaveLength(1);
      expectToEmit<Repaid>(res.events[0], 'Repaid', {
        account: signer,
        amount,
        asset,
        from: signer,
      });
    });
    it('repaid to another account', async () => {
      const res = await service.tx.repay(asset, amount, anotherAcconut);
      expect(res.events).toHaveLength(1);
      expectToEmit<Repaid>(res.events[0], 'Repaid', {
        account: anotherAcconut,
        amount,
        asset,
        from: signer,
      });
    });
  });
});
