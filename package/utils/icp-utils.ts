export const getEnumValue = <T>(object: object | undefined): T | undefined => {
    if (!object) {
        return undefined;
    }

    const keys = Object.keys(object);

    if (keys.length === 0) {
        return undefined;
    }

    // Return first key
    return keys[0] as T;
};
