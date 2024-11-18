export { ResourceSpec as ICPResourceSpec } from './types/ResourceSpec';
export { FileInfo, FileWithRole } from 'icp-declarations/storage/storage.did';

export { ICPIdentityDriver } from './drivers/ICPIdentityDriver';
export { ICPStorageDriver } from './drivers/ICPStorageDriver';
export {
    ICPSiweDriver,
    type SiweIdentityContextType,
    type LoginOkResponse,
    type SIWE_IDENTITY_SERVICE,
    type ISignedDelegation,
    type State
} from './drivers/ICPSiweDriver';

export { FileHelpers } from './utils/file-helpers';
