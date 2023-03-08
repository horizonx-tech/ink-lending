/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/service';
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
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	 * @param { ArgumentTypes.AccountId | null } to,
	*/
	"withdraw" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		to: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "service::withdraw", [asset, amount, to], __options);
	}

	/**
	 * liquidationCall
	 *
	 * @param { ArgumentTypes.AccountId } liquidatee,
	 * @param { ArgumentTypes.AccountId } collateralAsset,
	 * @param { ArgumentTypes.AccountId } debtAsset,
	 * @param { (string | number | BN) } debtAmount,
	*/
	"liquidationCall" (
		liquidatee: ArgumentTypes.AccountId,
		collateralAsset: ArgumentTypes.AccountId,
		debtAsset: ArgumentTypes.AccountId,
		debtAmount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "service::liquidationCall", [liquidatee, collateralAsset, debtAsset, debtAmount], __options);
	}

	/**
	 * deposit
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	 * @param { ArgumentTypes.AccountId | null } account,
	*/
	"deposit" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "service::deposit", [asset, amount, account], __options);
	}

	/**
	 * borrow
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	 * @param { ArgumentTypes.AccountId | null } account,
	*/
	"borrow" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "service::borrow", [asset, amount, account], __options);
	}

	/**
	 * repay
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	 * @param { ArgumentTypes.AccountId | null } account,
	*/
	"repay" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "service::repay", [asset, amount, account], __options);
	}

}