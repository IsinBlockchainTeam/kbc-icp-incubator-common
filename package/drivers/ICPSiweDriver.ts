import {
    type DerEncodedPublicKey,
    type Signature,
    type ActorMethod,
    HttpAgent,
    type ActorConfig,
    type HttpAgentOptions,
    Actor,
    type ActorSubclass
} from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import {
    DelegationChain,
    Delegation,
    type SignedDelegation,
    DelegationIdentity,
    Ed25519KeyIdentity
} from '@dfinity/identity';
import { idlFactory } from 'icp-declarations/ic_siwe_provider';

export type SiweIdentityContextType = {
    /** Is set to `true` on mount until a stored identity is loaded from local storage or
     * none is found. */
    isInitializing: boolean;

    /** Load a SIWE message from the provider canister, to be used for login. Calling prepareLogin
     * is optional, as it will be called automatically on login if not called manually. */
    prepareLogin: () => void;

    /** Reflects the current status of the prepareLogin process. */
    prepareLoginStatus: PrepareLoginStatus;

    /** `prepareLoginStatus === "loading"` */
    isPreparingLogin: boolean;

    /** `prepareLoginStatus === "error"` */
    isPrepareLoginError: boolean;

    /** `prepareLoginStatus === "success"` */
    isPrepareLoginSuccess: boolean;

    /** `prepareLoginStatus === "idle"` */
    isPrepareLoginIdle: boolean;

    /** Error that occurred during the prepareLogin process. */
    prepareLoginError?: Error;

    /** Initiates the login process by requesting a SIWE message from the backend. */
    login: () => Promise<DelegationIdentity | undefined>;

    /** Reflects the current status of the login process. */
    loginStatus: LoginStatus;

    /** `loginStatus === "logging-in"` */
    isLoggingIn: boolean;

    /** `loginStatus === "error"` */
    isLoginError: boolean;

    /** `loginStatus === "success"` */
    isLoginSuccess: boolean;

    /** `loginStatus === "idle"` */
    isLoginIdle: boolean;

    /** Error that occurred during the login process. */
    loginError?: Error;

    // /** Status of the SIWE message signing process. This is a re-export of the Wagmi
    //  * signMessage / status type. */
    // signMessageStatus: "error" | "idle" | "pending" | "success";
    //
    // /** Error that occurred during the SIWE message signing process. This is a re-export of the
    //  * Wagmi signMessage / error type. */
    // signMessageError: Error | null;

    /** The delegation chain is available after successfully loading the identity from local
     * storage or completing the login process. */
    delegationChain?: DelegationChain;

    /** The identity is available after successfully loading the identity from local storage
     * or completing the login process. */
    identity?: DelegationIdentity;

    /** The Ethereum address associated with current identity. This address is not necessarily
     * the same as the address of the currently connected wallet - on wallet change, the addresses
     * will differ. */
    identityAddress?: string;

    /** Clears the identity from the state and local storage. Effectively "logs the user out". */
    clear: () => void;
};

export type Address = string;

export type CanisterPublicKey = PublicKey;

export interface IDelegation {
    pubkey: PublicKey;
    targets: [] | [Array<Principal>];
    expiration: Timestamp;
}

export type GetDelegationResponse = { Ok: ISignedDelegation } | { Err: string };

export interface LoginOkResponse {
    user_canister_pubkey: CanisterPublicKey;
    expiration: Timestamp;
}

export type LoginResponse = { Ok: LoginOkResponse } | { Err: string };

export type PrepareLoginResponse = { Ok: SiweMessage } | { Err: string };

export type PublicKey = Uint8Array | number[];

export type SessionKey = PublicKey;

export interface ISignedDelegation {
    signature: Uint8Array | number[];
    delegation: IDelegation;
}

export type SiweMessage = string;

export type SiweSignature = string;

export type Timestamp = bigint;

export type SiweIdentityStorage = {
    address: string;
    sessionIdentity: string;
    delegationChain: string;
};

export interface SIWE_IDENTITY_SERVICE {
    siwe_prepare_login: ActorMethod<[Address], PrepareLoginResponse>;
    siwe_login: ActorMethod<[SiweSignature, Address, SessionKey], LoginResponse>;
    siwe_get_delegation: ActorMethod<[Address, SessionKey, Timestamp], GetDelegationResponse>;
}

export type PrepareLoginStatus = 'error' | 'preparing' | 'success' | 'idle';
export type LoginStatus = 'error' | 'logging-in' | 'success' | 'idle';

export type State = {
    anonymousActor?: ActorSubclass<SIWE_IDENTITY_SERVICE>;
    isInitializing: boolean;
    prepareLoginStatus: PrepareLoginStatus;
    prepareLoginError?: Error;
    siweMessage?: string;
    loginStatus: LoginStatus;
    loginError?: Error;
    identity?: DelegationIdentity;
    identityAddress?: string;
    delegationChain?: DelegationChain;
};

export class ICPSiweDriver {
    private readonly canisterId: string;
    private readonly STORAGE_KEY = 'siweIdentity';

    constructor(canisterId: string) {
        this.canisterId = canisterId;
    }

    // Delegation
    /**
     * Converts a Uint8Array or number array to a Signature object.
     */
    private asSignature(signature: Uint8Array | number[]): Signature {
        const arrayBuffer: ArrayBuffer = (signature as Uint8Array).buffer;
        const s: Signature = arrayBuffer as Signature;
        s.__signature__ = undefined;
        return s;
    }

    /**
     * Converts a Uint8Array or number array to a DerEncodedPublicKey object.
     */
    private asDerEncodedPublicKey(publicKey: Uint8Array | number[]): DerEncodedPublicKey {
        const arrayBuffer: ArrayBuffer = (publicKey as Uint8Array).buffer;
        const pk: DerEncodedPublicKey = arrayBuffer as DerEncodedPublicKey;
        pk.__derEncodedPublicKey__ = undefined;
        return pk;
    }

    createDelegationChain(signedDelegation: ISignedDelegation, publicKey: PublicKey) {
        const delegations: SignedDelegation[] = [
            {
                delegation: new Delegation(
                    (signedDelegation.delegation.pubkey as Uint8Array).buffer,
                    signedDelegation.delegation.expiration,
                    signedDelegation.delegation.targets[0] as Principal[]
                ),
                signature: this.asSignature(signedDelegation.signature)
            }
        ];
        return DelegationChain.fromDelegations(delegations, this.asDerEncodedPublicKey(publicKey));
    }

    normalizeError(error: Error | unknown): Error {
        return error instanceof Error ? error : new Error('An unknown error occurred.');
    }

    // Local Storage
    /**
     * Loads the SIWE identity from local storage.
     */
    loadIdentity() {
        const storedState = localStorage.getItem(this.STORAGE_KEY);

        if (!storedState) {
            throw new Error('No stored identity found.');
        }

        const s: SiweIdentityStorage = JSON.parse(storedState);
        if (!s.address || !s.sessionIdentity || !s.delegationChain) {
            throw new Error('Stored state is invalid.');
        }

        const d = DelegationChain.fromJSON(JSON.stringify(s.delegationChain));
        const i = DelegationIdentity.fromDelegation(
            Ed25519KeyIdentity.fromJSON(JSON.stringify(s.sessionIdentity)),
            d
        );

        return [s.address, i, d] as const;
    }

    /**
     * Saves the SIWE identity to local storage.
     */
    async saveIdentity(
        address: string,
        sessionIdentity: Ed25519KeyIdentity,
        delegationChain: DelegationChain
    ) {
        localStorage.setItem(
            this.STORAGE_KEY,
            JSON.stringify({
                address: address,
                sessionIdentity: sessionIdentity.toJSON(),
                delegationChain: delegationChain.toJSON()
            })
        );
    }

    /**
     * Clears the SIWE identity from local storage.
     */
    clearIdentity() {
        localStorage.removeItem(this.STORAGE_KEY);
    }

    // Siwe provider
    /**
     * Creates an anonymous actor for interactions with the Internet Computer.
     * This is used primarily for the initial authentication process.
     */
    createAnonymousActor({
        httpAgentOptions,
        actorOptions
    }: {
        httpAgentOptions?: HttpAgentOptions;
        actorOptions?: ActorConfig;
    }) {
        if (!idlFactory || !this.canisterId) return;
        const agent = new HttpAgent({ ...httpAgentOptions });

        if (process.env.DFX_NETWORK !== 'ic') {
            agent.fetchRootKey().catch((err) => {
                console.warn(
                    'Unable to fetch root key. Check to ensure that your local replica is running'
                );
                console.error(err);
            });
        }

        return Actor.createActor<SIWE_IDENTITY_SERVICE>(idlFactory, {
            agent,
            canisterId: this.canisterId,
            ...actorOptions
        });
    }

    async callPrepareLogin(
        anonymousActor: ActorSubclass<SIWE_IDENTITY_SERVICE>,
        address: `0x${string}` | undefined
    ) {
        if (!anonymousActor || !address) {
            throw new Error('Invalid actor or address');
        }

        const response = await anonymousActor.siwe_prepare_login(address);

        if ('Err' in response) {
            throw new Error(response.Err);
        }

        return response.Ok;
    }

    /**
     * Logs in the user by sending a signed SIWE message to the backend.
     */
    async callLogin(
        anonymousActor: ActorSubclass<SIWE_IDENTITY_SERVICE>,
        data: `0x${string}` | undefined,
        address: `0x${string}` | undefined,
        sessionPublicKey: DerEncodedPublicKey
    ) {
        if (!anonymousActor || !data || !address) {
            throw new Error('Invalid actor, data or address');
        }

        const loginReponse = await anonymousActor.siwe_login(
            data,
            address,
            new Uint8Array(sessionPublicKey)
        );

        if ('Err' in loginReponse) {
            throw new Error(loginReponse.Err);
        }

        return loginReponse.Ok;
    }

    /**
     * Retrieves a delegation from the backend for the current session.
     */
    async callGetDelegation(
        anonymousActor: ActorSubclass<SIWE_IDENTITY_SERVICE>,
        address: `0x${string}` | undefined,
        sessionPublicKey: DerEncodedPublicKey,
        expiration: bigint
    ) {
        if (!anonymousActor || !address) {
            throw new Error('Invalid actor or address');
        }

        const response = await anonymousActor.siwe_get_delegation(
            address,
            new Uint8Array(sessionPublicKey),
            expiration
        );

        if ('Err' in response) {
            throw new Error(response.Err);
        }

        return response.Ok;
    }
}
