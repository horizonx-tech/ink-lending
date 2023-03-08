/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/service';
import type * as ReturnTypes from '../types-returns/service';
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
	* withdraw
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId | null } to,
	* @returns { void }
	*/
	"withdraw" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		to: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "service::withdraw", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "service");
		}, [asset, amount, to], __options);
	}

	/**
	* liquidationCall
	*
	* @param { ArgumentTypes.AccountId } liquidatee,
	* @param { ArgumentTypes.AccountId } collateralAsset,
	* @param { ArgumentTypes.AccountId } debtAsset,
	* @param { (string | number | BN) } debtAmount,
	* @returns { void }
	*/
	"liquidationCall" (
		liquidatee: ArgumentTypes.AccountId,
		collateralAsset: ArgumentTypes.AccountId,
		debtAsset: ArgumentTypes.AccountId,
		debtAmount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "service::liquidationCall", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "service");
		}, [liquidatee, collateralAsset, debtAsset, debtAmount], __options);
	}

	/**
	* deposit
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId | null } account,
	* @returns { void }
	*/
	"deposit" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "service::deposit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "service");
		}, [asset, amount, account], __options);
	}

	/**
	* borrow
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId | null } account,
	* @returns { void }
	*/
	"borrow" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "service::borrow", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "service");
		}, [asset, amount, account], __options);
	}

	/**
	* repay
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId | null } account,
	* @returns { void }
	*/
	"repay" (
		asset: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		account: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "service::repay", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "service");
		}, [asset, amount, account], __options);
	}

}