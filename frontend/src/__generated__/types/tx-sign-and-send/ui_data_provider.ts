/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/ui_data_provider';
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
	* assets
	*
	*/
	"assets" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "assets", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "ui_data_provider");
		}, [], __options);
	}

	/**
	* pools
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	*/
	"pools" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "pools", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "ui_data_provider");
		}, [assets], __options);
	}

	/**
	* marketData
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	*/
	"marketData" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "marketData", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "ui_data_provider");
		}, [assets], __options);
	}

}