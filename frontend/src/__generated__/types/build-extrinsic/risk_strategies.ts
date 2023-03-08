/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/risk_strategies';
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
	 * validateWithdraw
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	*/
	"validateWithdraw" (
		account: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "riskStrategy::validateWithdraw", [account, asset, amount], __options);
	}

	/**
	 * validateLiquidation
	 *
	 * @param { ArgumentTypes.AccountId } liquidatee,
	 * @param { ArgumentTypes.AccountId } collateralAsset,
	 * @param { ArgumentTypes.AccountId } debtAsset,
	 * @param { (string | number | BN) } debtAmount,
	*/
	"validateLiquidation" (
		liquidatee: ArgumentTypes.AccountId,
		collateralAsset: ArgumentTypes.AccountId,
		debtAsset: ArgumentTypes.AccountId,
		debtAmount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "riskStrategy::validateLiquidation", [liquidatee, collateralAsset, debtAsset, debtAmount], __options);
	}

	/**
	 * validateBorrow
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } amount,
	*/
	"validateBorrow" (
		account: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "riskStrategy::validateBorrow", [account, asset, amount], __options);
	}

}