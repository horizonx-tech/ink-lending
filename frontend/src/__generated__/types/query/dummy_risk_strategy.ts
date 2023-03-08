/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dummy_risk_strategy';
import type * as ReturnTypes from '../types-returns/dummy_risk_strategy';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* setResult
	*
	* @param { boolean } result,
	* @returns { Result<null, ReturnTypes.LangError> }
	*/
	"setResult" (
		result: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setResult", [result], __options , (result) => { return handleReturnType(result, getTypeDescription(2, 'dummy_risk_strategy')); });
	}

	/**
	* setCollateralAmount
	*
	* @param { (string | number | BN) } collateralAmount,
	* @returns { Result<null, ReturnTypes.LangError> }
	*/
	"setCollateralAmount" (
		collateralAmount: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setCollateralAmount", [collateralAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(2, 'dummy_risk_strategy')); });
	}

	/**
	* validateLiquidation
	*
	* @param { ArgumentTypes.AccountId } liquidatee,
	* @param { ArgumentTypes.AccountId } collateralAsset,
	* @param { ArgumentTypes.AccountId } debtAsset,
	* @param { (string | number | BN) } debtAmount,
	* @returns { Result<Result<ReturnNumber, number>, ReturnTypes.LangError> }
	*/
	"validateLiquidation" (
		liquidatee: ArgumentTypes.AccountId,
		collateralAsset: ArgumentTypes.AccountId,
		debtAsset: ArgumentTypes.AccountId,
		debtAmount: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnNumber, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateLiquidation", [liquidatee, collateralAsset, debtAsset, debtAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'dummy_risk_strategy')); });
	}

	/**
	* validateWithdraw
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @returns { Result<Result<null, number>, ReturnTypes.LangError> }
	*/
	"validateWithdraw" (
		account: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateWithdraw", [account, asset, amount], __options , (result) => { return handleReturnType(result, getTypeDescription(10, 'dummy_risk_strategy')); });
	}

	/**
	* validateBorrow
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @returns { Result<Result<null, number>, ReturnTypes.LangError> }
	*/
	"validateBorrow" (
		account: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateBorrow", [account, asset, amount], __options , (result) => { return handleReturnType(result, getTypeDescription(10, 'dummy_risk_strategy')); });
	}

}