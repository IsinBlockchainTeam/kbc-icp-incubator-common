export const ROLES = {
    OWNER: 'Owner',
    EDITOR: 'Editor',
    VIEWER: 'Viewer'
} as const;

// Get the values of the ROLES object
export type Role = (typeof ROLES)[keyof typeof ROLES];
