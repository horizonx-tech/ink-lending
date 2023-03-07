import type { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";

import AssetPool_factory from '../types/constructors/asset_pool';
import SharesToken_factory from '../types/constructors/shares_token';
import RateStrategy_factory from '../types/constructors/rate_strategies';
import RiskStrategy_factory from '../types/constructors/risk_strategies';
import Service_factory from '../types/constructors/service';
import Factory_factory from '../types/constructors/factory';
import Registry_factory from '../types/constructors/registry';
import Manager_factory from '../types/constructors/manager';
import DummyPool_factory from '../types/constructors/dummy_pool';
import DummyRiskStrategy_factory from '../types/constructors/dummy_risk_strategy';
import PSP22Token_factory from '../types/constructors/psp22_token';

import AssetPool from '../types/contracts/asset_pool';
import SharesToken from '../types/contracts/shares_token';
import RateStrategy from '../types/contracts/rate_strategies';
import RiskStrategy from '../types/contracts/risk_strategies';
import Service from '../types/contracts/service';
import Factory from '../types/contracts/factory';
import Registry from '../types/contracts/registry';
import Manager from '../types/contracts/manager';
import DummyPool from '../types/contracts/dummy_pool';
import DummyRiskStrategy from '../types/contracts/dummy_risk_strategy';
import PSP22Token from '../types/contracts/psp22_token';

type FactoryArgs = {
  api: ApiPromise;
  signer: KeyringPair;
};

export const deployAssetPool = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<AssetPool_factory["new"]> }
) => {
  const factory = new AssetPool_factory(api, signer)
  const contract = await factory.new(...args)
  return new AssetPool(
    contract.address,
    signer,
    api
  )
}

export const deploySharesToken = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<SharesToken_factory["new"]> }
) => {
  const factory = new SharesToken_factory(api, signer)
  const contract = await factory.new(...args)
  return new SharesToken(
    contract.address,
    signer,
    api
  )
}

export const deployRateStrategy = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<RateStrategy_factory["new"]> }
) => {
  const factory = new RateStrategy_factory(api, signer)
  const contract = await factory.new(...args)
  return new RateStrategy(
    contract.address,
    signer,
    api
  )
}

export const deployRiskStrategy = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<RiskStrategy_factory["new"]> }
) => {
  const factory = new RiskStrategy_factory(api, signer)
  const contract = await factory.new(...args)
  return new RiskStrategy(
    contract.address,
    signer,
    api
  )
}

export const deployService = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<Service_factory["new"]> }
) => {
  const factory = new Service_factory(api, signer)
  const contract = await factory.new(...args)
  return new Service(
    contract.address,
    signer,
    api
  )
}

export const deployFactory = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<Factory_factory["new"]> }
) => {
  const factory = new Factory_factory(api, signer)
  const contract = await factory.new(...args)
  return new Factory(
    contract.address,
    signer,
    api
  )
}

export const deployRegistry = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<Registry_factory["new"]> }
) => {
  const factory = new Registry_factory(api, signer)
  const contract = await factory.new(...args)
  return new Registry(
    contract.address,
    signer,
    api
  )
}

export const deployManager = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<Manager_factory["new"]> }
) => {
  const factory = new Manager_factory(api, signer)
  const contract = await factory.new(...args)
  return new Manager(
    contract.address,
    signer,
    api
  )
}

// Mocks
export const deployDummyPool = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<DummyPool_factory["new"]> }
) => {
  const factory = new DummyPool_factory(api, signer)
  const contract = await factory.new(...args)
  return new DummyPool(
    contract.address,
    signer,
    api
  )
}

export const deployDummyRiskStrategy = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<DummyRiskStrategy_factory["new"]> }
) => {
  const factory = new DummyRiskStrategy_factory(api, signer)
  const contract = await factory.new(...args)
  return new DummyRiskStrategy(
    contract.address,
    signer,
    api
  )
}

export const deployPSP22Token = async (
  {
    api,
    signer,
    args
  }: FactoryArgs & { args: Parameters<PSP22Token_factory["new"]> }
) => {
  const factory = new PSP22Token_factory(api, signer)
  const contract = await factory.new(...args)
  return new PSP22Token(
    contract.address,
    signer,
    api
  )
}
