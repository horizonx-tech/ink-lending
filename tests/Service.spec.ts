import { expect } from '@jest/globals';
import { encodeAddress } from '@polkadot/keyring';
import Service_factory from '../types/constructors/service';
import Registry from '../types/contracts/registry';
import DummyPool from '../types/contracts/dummy_pool';
import DummyRiskStrategy from '../types/contracts/dummy_risk_strategy';
import Service from '../types/contracts/service';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { expectToEmit } from './testHelpers';
import {
  Borrowed,
  Deposited,
  Liquidated,
  Repaid,
  Withdrew,
} from 'event-types/service';
import {
  deployDummyPool,
  deployDummyRiskStrategy,
  deployRegistry,
} from './testContractsHelpers';

const zeroAddress = encodeAddress(
  '0x0000000000000000000000000000000000000000000000000000000000000000',
);

describe('Service spec', () => {
  let api: ApiPromise;
  let deployer: KeyringPair;
  let wallet: KeyringPair;

  let registry: Registry;
  let dummyPool: DummyPool;
  let service: Service;
  let dummmyRiskStrategy: DummyRiskStrategy;

  const asset = encodeAddress(
    '0x0000000000000000000000000000000000000000000000000000000000000001',
  );

  const setup = async (): Promise<void> => {
    ({ api, alice: deployer, bob: wallet } = globalThis.setup);

    registry = await deployRegistry({
      api,
      signer: deployer,
      args: [null],
    });

    dummyPool = await deployDummyPool({
      api,
      signer: deployer,
      args: [registry.address, asset, zeroAddress, zeroAddress],
    });

    dummmyRiskStrategy = await deployDummyRiskStrategy({
      api,
      signer: deployer,
      args: [],
    });

    service = new Service(
      (await new Service_factory(api, deployer).new(registry.address)).address,
      wallet,
      api,
    );

    // initialize
    await registry.tx.registerPool(asset, dummyPool.address);
    await registry.tx.setRiskStrategy(dummmyRiskStrategy.address, null);
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
    it('liquidated', async () => {
      const collateralAsset = asset;
      const debtAsset = asset;
      const collateralAmount = 99;
      const debtAmount = amount;

      await dummmyRiskStrategy.tx.setCollateralAmount(collateralAmount);

      const res = await service.tx.liquidationCall(
        anotherAcconut,
        collateralAsset,
        debtAsset,
        debtAmount,
      );
      expect(res.events).toHaveLength(1);
      expectToEmit<Liquidated>(res.events[0], 'Liquidated', {
        liquidator: signer,
        liquidatee: anotherAcconut,
        collateralAsset,
        debtAsset,
        collateralAmount,
        debtAmount,
      });
    });
  });
});
