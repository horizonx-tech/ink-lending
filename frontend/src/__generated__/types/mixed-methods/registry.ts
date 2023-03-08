/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/registry';
import type * as ReturnTypes from '../types-returns/registry';
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
	* manager
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"manager" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::manager", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* rateStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"rateStrategy" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::rateStrategy", [asset], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* pool
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"pool" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::pool", [asset], __options, (result) => { return handleReturnType(result, getTypeDescription(9, 'registry')); });
	}

	/**
	* assetsCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"assetsCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::assetsCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(10, 'registry')); });
	}

	/**
	* setFactory
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { void }
	*/
	"setFactory" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setFactory", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address], __options);
	}

	/**
	* priceOracle
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"priceOracle" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::priceOracle", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* factory
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"factory" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::factory", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* setRiskStrategy
	*
	* @param { ArgumentTypes.AccountId } address,
	* @param { ArgumentTypes.AccountId | null } asset,
	* @returns { void }
	*/
	"setRiskStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setRiskStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address, asset], __options);
	}

	/**
	* defaultRateStrategy
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"defaultRateStrategy" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::defaultRateStrategy", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* registerPool
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { ArgumentTypes.AccountId } pool,
	* @returns { void }
	*/
	"registerPool" (
		asset: ArgumentTypes.AccountId,
		pool: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::registerPool", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [asset, pool], __options);
	}

	/**
	* assetList
	*
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"assetList" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::assetList", [], __options, (result) => { return handleReturnType(result, getTypeDescription(15, 'registry')); });
	}

	/**
	* setPriceOracle
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { void }
	*/
	"setPriceOracle" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setPriceOracle", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address], __options);
	}

	/**
	* riskStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"riskStrategy" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::riskStrategy", [asset], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* defaultRiskStrategy
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"defaultRiskStrategy" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::defaultRiskStrategy", [], __options, (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* asset
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"asset" (
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::asset", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(9, 'registry')); });
	}

	/**
	* setManager
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { void }
	*/
	"setManager" (
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"setRateStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry::setRateStrategy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "registry");
		}, [address, asset], __options);
	}

}