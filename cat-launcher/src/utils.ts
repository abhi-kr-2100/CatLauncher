import { invoke } from "@tauri-apps/api/core";

export interface GameVariantInfo {
    name: string;
    description: string;
}

export async function fetchGameVariantsInfo(): Promise<GameVariantInfo[]> {
    const response = await invoke("get_game_variants_info");
    return response as GameVariantInfo[];
}
