/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/registry';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * manager
	 *
	*/
	"manager" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::manager", [], __options);
	}

	/**
	 * rateStrategy
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	*/
	"rateStrategy" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::rateStrategy", [asset], __options);
	}

	/**
	 * pool
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	*/
	"pool" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::pool", [asset], __options);
	}

	/**
	 * assetsCount
	 *
	*/
	"assetsCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::assetsCount", [], __options);
	}

	/**
	 * setFactory
	 *
	 * @param { ArgumentTypes.AccountId } address,
	*/
	"setFactory" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::setFactory", [address], __options);
	}

	/**
	 * priceOracle
	 *
	*/
	"priceOracle" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::priceOracle", [], __options);
	}

	/**
	 * factory
	 *
	*/
	"factory" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::factory", [], __options);
	}

	/**
	 * setRiskStrategy
	 *
	 * @param { ArgumentTypes.AccountId } address,
	 * @param { ArgumentTypes.AccountId | null } asset,
	*/
	"setRiskStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::setRiskStrategy", [address, asset], __options);
	}

	/**
	 * defaultRateStrategy
	 *
	*/
	"defaultRateStrategy" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::defaultRateStrategy", [], __options);
	}

	/**
	 * registerPool
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { ArgumentTypes.AccountId } pool,
	*/
	"registerPool" (
		asset: ArgumentTypes.AccountId,
		pool: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::registerPool", [asset, pool], __options);
	}

	/**
	 * assetList
	 *
	*/
	"assetList" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::assetList", [], __options);
	}

	/**
	 * setPriceOracle
	 *
	 * @param { ArgumentTypes.AccountId } address,
	*/
	"setPriceOracle" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::setPriceOracle", [address], __options);
	}

	/**
	 * riskStrategy
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	*/
	"riskStrategy" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::riskStrategy", [asset], __options);
	}

	/**
	 * defaultRiskStrategy
	 *
	*/
	"defaultRiskStrategy" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::defaultRiskStrategy", [], __options);
	}

	/**
	 * asset
	 *
	 * @param { (number | string | BN) } index,
	*/
	"asset" (
		index: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::asset", [index], __options);
	}

	/**
	 * setManager
	 *
	 * @param { ArgumentTypes.AccountId } address,
	*/
	"setManager" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::setManager", [address], __options);
	}

	/**
	 * setRateStrategy
	 *
	 * @param { ArgumentTypes.AccountId } address,
	 * @param { ArgumentTypes.AccountId | null } asset,
	*/
	"setRateStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry::setRateStrategy", [address, asset], __options);
	}

}