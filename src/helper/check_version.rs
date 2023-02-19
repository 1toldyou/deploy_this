pub fn is_same_major_or_minor_version(compare_to: &str) -> bool {
    let current_version = match semver::Version::parse(env!("CARGO_PKG_VERSION")) {
        Ok(v) => v,
        Err(e) => {
            error!("failed to parse current version: {}", e);
            return false;
        }
    };

    let compare_to_version = match semver::Version::parse(compare_to) {
        Ok(v) => v,
        Err(e) => {
            error!("failed to parse compare_to version: {}", e);
            return false;
        }
    };

    if current_version.major != compare_to_version.major {
        error!("major version is different");
        return false;
    }

    if current_version.minor != compare_to_version.minor {
        error!("minor version is different");
        return false;
    }

    return true;
}