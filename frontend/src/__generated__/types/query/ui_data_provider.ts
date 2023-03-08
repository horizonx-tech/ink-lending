/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/ui_data_provider';
import type * as ReturnTypes from '../types-returns/ui_data_provider';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
import ABI from '../../artifacts/ui_data_provider.json';


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
	* assets
	*
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"assets" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assets", [], __options , (result) => { return handleReturnType(result, getTypeDescription(6, 'ui_data_provider',ABI)); });
	}

	/**
	* pools
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	* @returns { Result<Array<ReturnTypes.PoolData>, ReturnTypes.LangError> }
	*/
	"pools" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.PoolData>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "pools", [assets], __options , (result) => { return handleReturnType(result, getTypeDescription(9, 'ui_data_provider',ABI)); });
	}

	/**
	* marketData
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	* @returns { Result<Array<ReturnTypes.MarketData>, ReturnTypes.LangError> }
	*/
	"marketData" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.MarketData>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "marketData", [assets], __options , (result) => { return handleReturnType(result, getTypeDescription(14, 'ui_data_provider',ABI)); });
	}

}