pub use libic::*;

pub async fn get_minecraft_versions() -> Vec<ShortVersion> {
    let resp =
        util::get("https://launchermeta.mojang.com/mc/game/version_manifest.json".to_string());

    let res: MinecraftVersionResult = serde_json::from_str(&resp).unwrap();
    let arr = res.versions.into_iter().map(|v| {
        let item = ShortVersion {
            id: v.id,
            r#type: v.r#type,
        };

        return item;
    });

    return arr.collect();
}
