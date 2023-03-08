import type BN from 'bn.js';

export type AccountId = string | number[]

export type Hash = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface factory_Error {
	poolImplementationMissing ? : null,
	callerIsNotManager ? : null,
	registry ? : registry_Error,
	custom ? : (number | string | BN)
}

export class factory_ErrorBuilder {
	static PoolImplementationMissing(): factory_Error {
		return {
			poolImplementationMissing: null,
		};
	}
	static CallerIsNotManager(): factory_Error {
		return {
			callerIsNotManager: null,
		};
	}
	static Registry(value: registry_Error): factory_Error {
		return {
			registry: value,
		};
	}
	static Custom(value: (number | string | BN)): factory_Error {
		return {
			custom: value,
		};
	}
}

export enum registry_Error {
	poolAlreadyExists = 'PoolAlreadyExists',
	callerIsNotManager = 'CallerIsNotManager'
}

