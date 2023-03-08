import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface Error {
	psp22 ? : PSP22Error,
	risk ? : (number | string | BN)
}

export class ErrorBuilder {
	static PSP22(value: PSP22Error): Error {
		return {
			psp22: value,
		};
	}
	static Risk(value: (number | string | BN)): Error {
		return {
			risk: value,
		};
	}
}

export interface PSP22Error {
	custom ? : Array<(number | string | BN)>,
	insufficientBalance ? : null,
	insufficientAllowance ? : null,
	zeroRecipientAddress ? : null,
	zeroSenderAddress ? : null,
	safeTransferCheckFailed ? : Array<(number | string | BN)>
}

export class PSP22ErrorBuilder {
	static Custom(value: Array<(number | string | BN)>): PSP22Error {
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
	static SafeTransferCheckFailed(value: Array<(number | string | BN)>): PSP22Error {
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
	liquidityShare: (string | number | BN),
	liquidityIndex: (string | number | BN),
	liquidityRate: (string | number | BN),
	debtShare: (string | number | BN),
	debtIndex: (string | number | BN),
	debtRate: (string | number | BN),
	lastUpdateTimestamp: (number | string | BN)
}

