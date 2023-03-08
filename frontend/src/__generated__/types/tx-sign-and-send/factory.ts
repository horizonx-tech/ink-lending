/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/factory';
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
	* registry
	*
	*/
	"registry" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "registry", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "factory");
		}, [], __options);
	}

	/**
	* poolCodeHash
	*
	*/
	"poolCodeHash" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "poolCodeHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "factory");
		}, [], __options);
	}

	/**
	* sharesCodeHash
	*
	*/
	"sharesCodeHash" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "sharesCodeHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "factory");
		}, [], __options);
	}

	/**
	* create
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { Array<(number | string | BN)> } data,
	*/
	"create" (
		asset: ArgumentTypes.AccountId,
		data: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "factory::create", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "factory");
		}, [asset, data], __options);
	}

}