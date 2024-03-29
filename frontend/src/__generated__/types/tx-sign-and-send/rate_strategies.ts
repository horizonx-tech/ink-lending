/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rate_strategies';
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
	* calculateRate
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } liquidityAdded,
	* @param { (string | number | BN) } liquidityTaken,
	*/
	"calculateRate" (
		asset: ArgumentTypes.AccountId,
		liquidityAdded: (string | number | BN),
		liquidityTaken: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "rateStrategy::calculateRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "rate_strategies");
		}, [asset, liquidityAdded, liquidityTaken], __options);
	}

}