/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dummy_risk_strategy';
import type * as ReturnTypes from '../types-returns/dummy_risk_strategy';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __callerAddress : string;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* setResult
	*
	* @param { boolean } result,
	* @returns { void }
	*/
	"setResult" (
		result: boolean,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setResult", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "dummy_risk_strategy");
		}, [result], __options);
	}

	/**
	* setCollateralAmount
	*
	* @param { (string | number | BN) } collateralAmount,
	* @returns { void }
	*/
	"setCollateralAmount" (
		collateralAmount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setCollateralAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "dummy_risk_strategy");
		}, [collateralAmount], __options);
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnNumber, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateLiquidation", [liquidatee, collateralAsset, debtAsset, debtAmount], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'dummy_risk_strategy')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Result<null, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateWithdraw", [account, asset, amount], __options, (result) => { return handleReturnType(result, getTypeDescription(10, 'dummy_risk_strategy')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Result<null, number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "riskStrategy::validateBorrow", [account, asset, amount], __options, (result) => { return handleReturnType(result, getTypeDescription(10, 'dummy_risk_strategy')); });
	}

}