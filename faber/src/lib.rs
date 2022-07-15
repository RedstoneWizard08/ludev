pub use libic::*;

pub async fn get_fabric_versions() -> Vec<String> {
    let resp = util::get(
        "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml".to_string(),
    );
    let raw = xmltojson::to_json(&resp).expect("Something went wrong.");
    let serialized = serde_json::to_string(&raw).unwrap();
    let des: RawMavenVersionResult = serde_json::from_str(&serialized).unwrap();
    return des.metadata.versioning.versions.version;
}
