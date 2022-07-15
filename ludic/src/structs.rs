pub struct ModLoader {
    /// The mod loader's name. Will be "vanilla," "forge," "quilt," or "fabric".
    pub name: String,

    /// The mod loader's version.
    pub version: String,

    /// The mod loader's download URL.
    pub url: String,

    /// The mod loader's download file name.
    pub file_name: String,

    /// The mod loader's download file size.
    pub file_size: String,

    /// The mod loader's launch command.
    pub launch_command: Vec<String>,
}
