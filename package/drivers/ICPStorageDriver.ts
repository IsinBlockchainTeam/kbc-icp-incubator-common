import { createActor } from 'icp-declarations/storage';
import { ResourceSpec } from '../types/ResourceSpec';
import FileHelpers from '../utils/file-helpers';
import { PromisePool } from '@supercharge/promise-pool';
import { _SERVICE, FileInfo } from 'icp-declarations/storage/storage.did';
import type { ActorSubclass, Identity } from '@dfinity/agent';

export class ICPStorageDriver {
    private readonly _actor: ActorSubclass<_SERVICE>;

    public constructor(icpIdentity: Identity, canisterId: string, host?: string) {
        if (!icpIdentity) throw new Error('ICPStorageDriver: No ICP identity found');

        this._actor = createActor(canisterId, {
            agentOptions: {
                identity: icpIdentity,
                ...(host && { host })
            }
        });
    }

    async create(
        bytes: Uint8Array,
        resourceSpec: ResourceSpec,
        delegatedOrganizationIds: number[] = []
    ): Promise<number> {
        const { name, type } = resourceSpec;
        const hash: Uint8Array = FileHelpers.getHash(bytes);
        const chunks: any[] = FileHelpers.getChunks(bytes);

        const organizationId = parseInt(name.split('/')[4]);
        const delegatedIds = delegatedOrganizationIds ? delegatedOrganizationIds.map(BigInt) : [];

        const fileId = await this._actor.create_file(
            BigInt(organizationId),
            delegatedIds,
            name,
            type,
            BigInt(bytes.length),
            hash
        );

        // Create a promise pool to upload chunks in parallel
        const { errors } = await PromisePool.withConcurrency(20)
            .for(chunks)
            .process(async (chunk, index) => {
                return this._actor.put_chunk(fileId, index, chunk);
            });

        if (errors.length > 0)
            throw new Error(`ICPStorageDriver: Error uploading chunks: ${errors}`);

        return fileId;
    }

    async listFiles(organizationId: number) {
        return await this._actor.get_files_for_organization(BigInt(organizationId));
    }

    async getFile(file: FileInfo): Promise<Uint8Array> {
        // Create a download promise pool
        let { results } = await PromisePool.withConcurrency(10)
            .for(Array.from({ length: Number(file.chunks) }, (_, i) => i))
            .process(async (index) => {
                const chunk = await this._actor.get_file_chunk(file.id, index);

                if ('Ok' in chunk) {
                    return {
                        index,
                        chunk: chunk.Ok
                    };
                }
            });

        results = results.sort((a, b) => a!.index - b!.index);

        // Keep only the chunks
        let res = results.map((result) => result!.chunk.chunk);

        // Merge the chunks into a single buffer
        const total = res.reduce((acc, chunk) => acc + chunk!.length, 0);
        const buffer = new Uint8Array(total);
        let offset = 0;
        for (const chunk of res) {
            buffer.set(chunk!, offset);
            offset += chunk!.length;
        }

        return buffer;
    }

    async renameFile(fileId: number, newName: string) {
        await this._actor.rename_file(fileId, newName);
    }

    async deleteFile(fileId: number) {
        await this._actor.remove_file(fileId);
    }
}
