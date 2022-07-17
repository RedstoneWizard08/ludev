use std::collections::HashMap;

pub use libic::*;

pub async fn get_forge_versions() -> HashMap<String, Vec<String>> {
    let resp = util::get(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml".to_string(),
    );
    let raw = xmltojson::to_json(&resp).expect("Something went wrong.");
    let serialized = serde_json::to_string(&raw).unwrap();
    let des: RawMavenVersionResult = serde_json::from_str(&serialized).unwrap();
    let versions = des.metadata.versioning.versions.version;
    let versions_iter = versions.into_iter();
    
    let mut organized_versions: HashMap<String, Vec<String>> = HashMap::new();
    let mut mc_versions: Vec<String> = Vec::new();

    versions_iter.for_each(|v| {
        let mcv = v.split("-").into_iter().nth(0).unwrap();
        
        if !mc_versions.contains(&mcv.to_string()) {
            mc_versions.push(mcv.to_string());
        }

        let forge_raw = v.split("-").into_iter();
        let mut forge_raw_vec: Vec<String> = Vec::new();

        forge_raw.for_each(|f| {
            if f != mcv {
                forge_raw_vec.push(f.to_string());
            }
        });

        let forge_version = forge_raw_vec.join("-");

        if !organized_versions.contains_key(&mcv.to_string()) {
            organized_versions.insert(mcv.to_string(), Vec::new());
        }

        organized_versions.get_mut(&mcv.to_string()).unwrap().push(forge_version);
    });

    return organized_versions;
}
