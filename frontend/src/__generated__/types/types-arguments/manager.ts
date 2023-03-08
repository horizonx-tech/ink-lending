import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface manager_Error {
	factory ? : factory_Error,
	registry ? : registry_Error,
	accessControl ? : AccessControlError
}

export class manager_ErrorBuilder {
	static Factory(value: factory_Error): manager_Error {
		return {
			factory: value,
		};
	}
	static Registry(value: registry_Error): manager_Error {
		return {
			registry: value,
		};
	}
	static AccessControl(value: AccessControlError): manager_Error {
		return {
			accessControl: value,
		};
	}
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

export enum AccessControlError {
	invalidCaller = 'InvalidCaller',
	missingRole = 'MissingRole',
	roleRedundant = 'RoleRedundant'
}

