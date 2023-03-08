/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/factory';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * registry
	 *
	*/
	"registry" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "registry", [], __options);
	}

	/**
	 * poolCodeHash
	 *
	*/
	"poolCodeHash" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "poolCodeHash", [], __options);
	}

	/**
	 * sharesCodeHash
	 *
	*/
	"sharesCodeHash" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "sharesCodeHash", [], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "factory::create", [asset, data], __options);
	}

}