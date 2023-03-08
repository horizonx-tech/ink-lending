/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/asset_pool';
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::withdraw", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, to, amount], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::transferCollateralOnLiquidation", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [liquidatee, receiver, amount], __options);
	}

	/**
	* registry
	*
	*/
	"registry" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::registry", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::deposit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, from, amount], __options);
	}

	/**
	* liquidityRate
	*
	*/
	"liquidityRate" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::liquidityRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* lastUpdateTimestamp
	*
	*/
	"lastUpdateTimestamp" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::lastUpdateTimestamp", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* debtToken
	*
	*/
	"debtToken" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::debtToken", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::repay", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, from, amount], __options);
	}

	/**
	* asset
	*
	*/
	"asset" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::asset", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* liquidityIndex
	*
	*/
	"liquidityIndex" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::liquidityIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* debtIndex
	*
	*/
	"debtIndex" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::debtIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::borrow", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, to, amount], __options);
	}

	/**
	* debtRate
	*
	*/
	"debtRate" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::debtRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* collateralToken
	*
	*/
	"collateralToken" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::collateralToken", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

	/**
	* poolData
	*
	*/
	"poolData" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "uiPoolDataProvider::poolData", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [], __options);
	}

}