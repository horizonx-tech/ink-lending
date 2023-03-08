/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dummy_oracle';
import type * as ReturnTypes from '../types-returns/dummy_oracle';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* setPrice
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { (string | number | BN) } price,
	* @returns { Result<Result<null, null>, ReturnTypes.LangError> }
	*/
	"setPrice" (
		asset: ArgumentTypes.AccountId,
		price: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, null>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setPrice", [asset, price], __options , (result) => { return handleReturnType(result, getTypeDescription(7, 'dummy_oracle')); });
	}

	/**
	* get
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"get" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "priceOracle::get", [asset], __options , (result) => { return handleReturnType(result, getTypeDescription(9, 'dummy_oracle')); });
	}

}