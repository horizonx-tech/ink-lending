/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rate_strategies';
import type * as ReturnTypes from '../types-returns/rate_strategies';
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
	* calculateRate
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } liquidityAdded,
	* @param { (string | number | BN) } liquidityTaken,
	* @returns { Result<[ReturnNumber, ReturnNumber], ReturnTypes.LangError> }
	*/
	"calculateRate" (
		asset: ArgumentTypes.AccountId,
		liquidityAdded: (string | number | BN),
		liquidityTaken: (string | number | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<[ReturnNumber, ReturnNumber], ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "rateStrategy::calculateRate", [asset, liquidityAdded, liquidityTaken], __options, (result) => { return handleReturnType(result, getTypeDescription(7, 'rate_strategies')); });
	}

}