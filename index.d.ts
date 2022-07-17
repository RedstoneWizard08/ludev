interface ForgeVersionsResult {
    [key: string]: string[];
}

export function getFabricVersions(): Promise<string[]>;
export function getForgeVersions(): Promise<ForgeVersionsResult>;
