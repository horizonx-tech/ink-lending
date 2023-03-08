/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dummy_oracle';
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
	 * setPrice
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } price,
	*/
	"setPrice" (
		asset: ArgumentTypes.AccountId,
		price: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setPrice", [asset, price], __options);
	}

	/**
	 * get
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	*/
	"get" (
		asset: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "priceOracle::get", [asset], __options);
	}

}