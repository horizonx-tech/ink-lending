/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/asset_pool';
import type * as ReturnTypes from '../types-returns/asset_pool';
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
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } to,
	* @param { (string | number | BN) } amount,
	* @returns { void }
	*/
	"withdraw" (
		account: ArgumentTypes.AccountId,
		to: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
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
	* @returns { void }
	*/
	"transferCollateralOnLiquidation" (
		liquidatee: ArgumentTypes.AccountId,
		receiver: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::transferCollateralOnLiquidation", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [liquidatee, receiver, amount], __options);
	}

	/**
	* registry
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"registry" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::registry", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, 'asset_pool')); });
	}

	/**
	* deposit
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } from,
	* @param { (string | number | BN) } amount,
	* @returns { void }
	*/
	"deposit" (
		account: ArgumentTypes.AccountId,
		from: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::deposit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, from, amount], __options);
	}

	/**
	* liquidityRate
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"liquidityRate" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::liquidityRate", [], __options, (result) => { return handleReturnType(result, getTypeDescription(14, 'asset_pool')); });
	}

	/**
	* lastUpdateTimestamp
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"lastUpdateTimestamp" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::lastUpdateTimestamp", [], __options, (result) => { return handleReturnType(result, getTypeDescription(15, 'asset_pool')); });
	}

	/**
	* debtToken
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"debtToken" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::debtToken", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, 'asset_pool')); });
	}

	/**
	* repay
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } from,
	* @param { (string | number | BN) } amount,
	* @returns { void }
	*/
	"repay" (
		account: ArgumentTypes.AccountId,
		from: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::repay", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, from, amount], __options);
	}

	/**
	* asset
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"asset" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::asset", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, 'asset_pool')); });
	}

	/**
	* liquidityIndex
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"liquidityIndex" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::liquidityIndex", [], __options, (result) => { return handleReturnType(result, getTypeDescription(14, 'asset_pool')); });
	}

	/**
	* debtIndex
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"debtIndex" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::debtIndex", [], __options, (result) => { return handleReturnType(result, getTypeDescription(14, 'asset_pool')); });
	}

	/**
	* borrow
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.AccountId } to,
	* @param { (string | number | BN) } amount,
	* @returns { void }
	*/
	"borrow" (
		account: ArgumentTypes.AccountId,
		to: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assetPool::borrow", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "asset_pool");
		}, [account, to, amount], __options);
	}

	/**
	* debtRate
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"debtRate" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::debtRate", [], __options, (result) => { return handleReturnType(result, getTypeDescription(14, 'asset_pool')); });
	}

	/**
	* collateralToken
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"collateralToken" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assetPool::collateralToken", [], __options, (result) => { return handleReturnType(result, getTypeDescription(13, 'asset_pool')); });
	}

	/**
	* poolData
	*
	* @returns { Result<ReturnTypes.PoolData, ReturnTypes.LangError> }
	*/
	"poolData" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.PoolData, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "uiPoolDataProvider::poolData", [], __options, (result) => { return handleReturnType(result, getTypeDescription(16, 'asset_pool')); });
	}

}