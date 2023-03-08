/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/ui_data_provider';
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
	 * assets
	 *
	*/
	"assets" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "assets", [], __options);
	}

	/**
	 * pools
	 *
	 * @param { Array<ArgumentTypes.AccountId> | null } assets,
	*/
	"pools" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "pools", [assets], __options);
	}

	/**
	 * marketData
	 *
	 * @param { Array<ArgumentTypes.AccountId> | null } assets,
	*/
	"marketData" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "marketData", [assets], __options);
	}

}