import CryptoJS from 'crypto-js';
import { Buffer } from 'buffer';

const DEFAULT_CHUNK_SIZE = 1900000; // 1.9MB

export type SupportedAlgorithm =
    | 'MD5'
    | 'SHA1'
    | 'SHA256'
    | 'SHA224'
    | 'SHA512'
    | 'SHA384'
    | 'SHA3'
    | 'RIPEMD160';

export class FileHelpers {
    public static computeHashFromBuffer(
        buffer: Buffer,
        algorithm: SupportedAlgorithm = 'SHA256'
    ): string {
        const wordArray = CryptoJS.lib.WordArray.create(buffer);
        const hash = CryptoJS.algo[algorithm].create().finalize(wordArray);
        return hash.toString(CryptoJS.enc.Hex);
    }

    public static formatBytes(bytes: number, decimals = 2) {
        if (!+bytes) return '0 Bytes';

        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
    }

    public static getHash(bytes: Uint8Array): Uint8Array {
        const wordArray = CryptoJS.lib.WordArray.create(bytes);
        const hash = CryptoJS.SHA256(wordArray);
        const hashBuffer = Buffer.from(hash.toString(CryptoJS.enc.Hex), 'hex');
        return new Uint8Array(hashBuffer);
    }

    public static getChunks(bytes: Uint8Array, chunkSize: number = DEFAULT_CHUNK_SIZE) {
        const chunks = [];

        for (let i = 0; i < bytes.byteLength; i += chunkSize) {
            const chunk: Uint8Array = new Uint8Array(bytes.slice(i, i + chunkSize));
            const hash = FileHelpers.computeHashFromBuffer(Buffer.from(chunk));
            chunks.push({
                chunk,
                hash: new Uint8Array(Buffer.from(hash, 'hex'))
            });
        }

        return chunks;
    }

    public static getBytesFromObject(object: object): Uint8Array {
        return new TextEncoder().encode(JSON.stringify(object));
    }

    public static getObjectFromBytes(bytes: Uint8Array): object {
        return JSON.parse(new TextDecoder().decode(bytes));
    }

    public static removeFileExtension(file: string): string {
        return file.split('.').slice(0, -1).join('.') || file;
    }

    public static ensureTrailingSlash(url: string): string {
        return url.endsWith('/') ? url : `${url}/`;
    }
}

export default FileHelpers;
