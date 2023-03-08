/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/asset_pool';
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
	 * withdraw
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (string | number | BN) } amount,
	*/
	"withdraw" (
		account: ArgumentTypes.AccountId,
		to: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::withdraw", [account, to, amount], __options);
	}

	/**
	 * transferCollateralOnLiquidation
	 *
	 * @param { ArgumentTypes.AccountId } liquidatee,
	 * @param { ArgumentTypes.AccountId } receiver,
	 * @param { (string | number | BN) } amount,
	*/
	"transferCollateralOnLiquidation" (
		liquidatee: ArgumentTypes.AccountId,
		receiver: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::transferCollateralOnLiquidation", [liquidatee, receiver, amount], __options);
	}

	/**
	 * registry
	 *
	*/
	"registry" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::registry", [], __options);
	}

	/**
	 * deposit
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } from,
	 * @param { (string | number | BN) } amount,
	*/
	"deposit" (
		account: ArgumentTypes.AccountId,
		from: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::deposit", [account, from, amount], __options);
	}

	/**
	 * liquidityRate
	 *
	*/
	"liquidityRate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::liquidityRate", [], __options);
	}

	/**
	 * lastUpdateTimestamp
	 *
	*/
	"lastUpdateTimestamp" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::lastUpdateTimestamp", [], __options);
	}

	/**
	 * debtToken
	 *
	*/
	"debtToken" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::debtToken", [], __options);
	}

	/**
	 * repay
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } from,
	 * @param { (string | number | BN) } amount,
	*/
	"repay" (
		account: ArgumentTypes.AccountId,
		from: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::repay", [account, from, amount], __options);
	}

	/**
	 * asset
	 *
	*/
	"asset" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::asset", [], __options);
	}

	/**
	 * liquidityIndex
	 *
	*/
	"liquidityIndex" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::liquidityIndex", [], __options);
	}

	/**
	 * debtIndex
	 *
	*/
	"debtIndex" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::debtIndex", [], __options);
	}

	/**
	 * borrow
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (string | number | BN) } amount,
	*/
	"borrow" (
		account: ArgumentTypes.AccountId,
		to: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::borrow", [account, to, amount], __options);
	}

	/**
	 * debtRate
	 *
	*/
	"debtRate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::debtRate", [], __options);
	}

	/**
	 * collateralToken
	 *
	*/
	"collateralToken" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assetPool::collateralToken", [], __options);
	}

	/**
	 * poolData
	 *
	*/
	"poolData" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "uiPoolDataProvider::poolData", [], __options);
	}

}