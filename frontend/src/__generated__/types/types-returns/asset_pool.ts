import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface Error {
	psp22 ? : PSP22Error,
	risk ? : number
}

export class ErrorBuilder {
	static PSP22(value: PSP22Error): Error {
		return {
			psp22: value,
		};
	}
	static Risk(value: number): Error {
		return {
			risk: value,
		};
	}
}

export interface PSP22Error {
	custom ? : Array<number>,
	insufficientBalance ? : null,
	insufficientAllowance ? : null,
	zeroRecipientAddress ? : null,
	zeroSenderAddress ? : null,
	safeTransferCheckFailed ? : Array<number>
}

export class PSP22ErrorBuilder {
	static Custom(value: Array<number>): PSP22Error {
		return {
			custom: value,
		};
	}
	static InsufficientBalance(): PSP22Error {
		return {
			insufficientBalance: null,
		};
	}
	static InsufficientAllowance(): PSP22Error {
		return {
			insufficientAllowance: null,
		};
	}
	static ZeroRecipientAddress(): PSP22Error {
		return {
			zeroRecipientAddress: null,
		};
	}
	static ZeroSenderAddress(): PSP22Error {
		return {
			zeroSenderAddress: null,
		};
	}
	static SafeTransferCheckFailed(value: Array<number>): PSP22Error {
		return {
			safeTransferCheckFailed: value,
		};
	}
}

export type PoolData = {
	registry: AccountId,
	asset: AccountId,
	collateralToken: AccountId,
	debtToken: AccountId,
	liquidityShare: ReturnNumber,
	liquidityIndex: ReturnNumber,
	liquidityRate: ReturnNumber,
	debtShare: ReturnNumber,
	debtIndex: ReturnNumber,
	debtRate: ReturnNumber,
	lastUpdateTimestamp: number
}

