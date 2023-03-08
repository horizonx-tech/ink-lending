/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/risk_strategies';
import type BN from 'bn.js';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise: ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "riskStrategy::validateWithdraw", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "risk_strategies");
		}, [account, asset, amount], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "riskStrategy::validateLiquidation", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "risk_strategies");
		}, [liquidatee, collateralAsset, debtAsset, debtAmount], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "riskStrategy::validateBorrow", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "risk_strategies");
		}, [account, asset, amount], __options);
	}

}