/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/registry';
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
	* manager
	*
	*/
	"manager" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::manager", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
	}

	/**
	* rateStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	*/
	"rateStrategy" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::rateStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [asset], __options);
	}

	/**
	* pool
	*
	* @param { ArgumentTypes.AccountId } asset,
	*/
	"pool" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::pool", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [asset], __options);
	}

	/**
	* assetsCount
	*
	*/
	"assetsCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::assetsCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
	}

	/**
	* setFactory
	*
	* @param { ArgumentTypes.AccountId } address,
	*/
	"setFactory" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setFactory", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address], __options);
	}

	/**
	* priceOracle
	*
	*/
	"priceOracle" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::priceOracle", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
	}

	/**
	* factory
	*
	*/
	"factory" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::factory", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setRiskStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address, asset], __options);
	}

	/**
	* defaultRateStrategy
	*
	*/
	"defaultRateStrategy" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::defaultRateStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::registerPool", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [asset, pool], __options);
	}

	/**
	* assetList
	*
	*/
	"assetList" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::assetList", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
	}

	/**
	* setPriceOracle
	*
	* @param { ArgumentTypes.AccountId } address,
	*/
	"setPriceOracle" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setPriceOracle", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address], __options);
	}

	/**
	* riskStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	*/
	"riskStrategy" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::riskStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [asset], __options);
	}

	/**
	* defaultRiskStrategy
	*
	*/
	"defaultRiskStrategy" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::defaultRiskStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [], __options);
	}

	/**
	* asset
	*
	* @param { (number | string | BN) } index,
	*/
	"asset" (
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::asset", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [index], __options);
	}

	/**
	* setManager
	*
	* @param { ArgumentTypes.AccountId } address,
	*/
	"setManager" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setManager", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setRateStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address, asset], __options);
	}

}