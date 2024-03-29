/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dummy_oracle';
import type * as ReturnTypes from '../types-returns/dummy_oracle';
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
	* setPrice
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } price,
	* @returns { void }
	*/
	"setPrice" (
		asset: ArgumentTypes.AccountId,
		price: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setPrice", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "dummy_oracle");
		}, [asset, price], __options);
	}

	/**
	* get
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"get" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "priceOracle::get", [asset], __options, (result) => { return handleReturnType(result, getTypeDescription(9, 'dummy_oracle')); });
	}

}