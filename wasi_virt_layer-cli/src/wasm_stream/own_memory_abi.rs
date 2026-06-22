pub const OWN_MEMORY_SIZE_PREFIX: &str = "__wasip1_vfs_own_memory_size_";
pub const OWN_MEMORY_GROW_PREFIX: &str = "__wasip1_vfs_own_memory_grow_";

pub const HOST_OWN_MEMORY_SIZE_GET: &str = "__wasip1_vfs_host_own_memory_size_get";
pub const HOST_OWN_MEMORY_SIZE_SET: &str = "__wasip1_vfs_host_own_memory_size_set";
pub const HOST_OWN_MEMORY_SIZE_INIT: &str = "__wasip1_vfs_host_own_memory_size_init";
pub const HOST_OWN_MEMORY_SIZE_COMPARE_EXCHANGE: &str =
    "__wasip1_vfs_host_own_memory_size_compare_exchange";

pub const TARGET_OWN_MEMORY_SIZE_GET_SUFFIX: &str = "_own_memory_size_get";
pub const TARGET_OWN_MEMORY_SIZE_SET_SUFFIX: &str = "_own_memory_size_set";
pub const TARGET_OWN_MEMORY_SIZE_INIT_SUFFIX: &str = "_own_memory_size_init";
pub const TARGET_OWN_MEMORY_SIZE_COMPARE_EXCHANGE_SUFFIX: &str =
    "_own_memory_size_compare_exchange";
pub const TARGET_MEMORY_COPY_FROM_SUFFIX: &str = "_memory_copy_from";
pub const TARGET_MEMORY_COPY_TO_SUFFIX: &str = "_memory_copy_to";

const MEMORY_DIRECTOR_PREFIX: &str = "__wasip1_vfs_";
const MEMORY_DIRECTOR_SUFFIX: &str = "_memory_director";

pub fn sanitize_target_name(target: &str) -> String {
    target.replace('-', "_")
}

pub fn own_memory_size_import_name(target: &str) -> String {
    format!("{OWN_MEMORY_SIZE_PREFIX}{}", sanitize_target_name(target))
}

pub fn own_memory_grow_import_name(target: &str) -> String {
    format!("{OWN_MEMORY_GROW_PREFIX}{}", sanitize_target_name(target))
}

pub fn parse_own_memory_size_import(name: &str) -> Option<&str> {
    name.strip_prefix(OWN_MEMORY_SIZE_PREFIX)
}

pub fn parse_own_memory_grow_import(name: &str) -> Option<&str> {
    name.strip_prefix(OWN_MEMORY_GROW_PREFIX)
}

pub fn parse_own_memory_import_target(name: &str) -> Option<&str> {
    parse_own_memory_size_import(name).or_else(|| parse_own_memory_grow_import(name))
}

pub fn memory_director_export_name(target: &str) -> String {
    format!(
        "{MEMORY_DIRECTOR_PREFIX}{}{MEMORY_DIRECTOR_SUFFIX}",
        sanitize_target_name(target)
    )
}

pub fn parse_memory_director_export(name: &str) -> Option<&str> {
    name.strip_prefix(MEMORY_DIRECTOR_PREFIX)
        .and_then(|name| name.strip_suffix(MEMORY_DIRECTOR_SUFFIX))
}

pub fn parse_prefixed_target_export<'a>(name: &'a str, suffix: &str) -> Option<&'a str> {
    name.strip_prefix(MEMORY_DIRECTOR_PREFIX)
        .and_then(|name| name.strip_suffix(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_and_parses_own_memory_import_names() {
        assert_eq!(
            own_memory_size_import_name("__self"),
            "__wasip1_vfs_own_memory_size___self"
        );
        assert_eq!(
            own_memory_grow_import_name("target-wasm"),
            "__wasip1_vfs_own_memory_grow_target_wasm"
        );

        assert_eq!(
            parse_own_memory_size_import("__wasip1_vfs_own_memory_size___self"),
            Some("__self")
        );
        assert_eq!(
            parse_own_memory_grow_import("__wasip1_vfs_own_memory_grow_target_wasm"),
            Some("target_wasm")
        );
        assert_eq!(
            parse_own_memory_import_target("__wasip1_vfs_own_memory_size_target_wasm"),
            Some("target_wasm")
        );
        assert_eq!(parse_own_memory_import_target("fd_write"), None);
    }

    #[test]
    fn formats_and_parses_memory_director_exports() {
        assert_eq!(sanitize_target_name("target-wasm"), "target_wasm");
        assert_eq!(
            memory_director_export_name("target-wasm"),
            "__wasip1_vfs_target_wasm_memory_director"
        );
        assert_eq!(
            parse_memory_director_export("__wasip1_vfs_target_wasm_memory_director"),
            Some("target_wasm")
        );
        assert_eq!(
            parse_memory_director_export("target_wasm_memory_director"),
            None
        );
        assert_eq!(
            parse_memory_director_export("__wasip1_vfs_target_wasm"),
            None
        );
    }

    #[test]
    fn exposes_host_own_memory_size_export_names() {
        assert_eq!(
            HOST_OWN_MEMORY_SIZE_GET,
            "__wasip1_vfs_host_own_memory_size_get"
        );
        assert_eq!(
            HOST_OWN_MEMORY_SIZE_SET,
            "__wasip1_vfs_host_own_memory_size_set"
        );
        assert_eq!(
            HOST_OWN_MEMORY_SIZE_INIT,
            "__wasip1_vfs_host_own_memory_size_init"
        );
        assert_eq!(
            HOST_OWN_MEMORY_SIZE_COMPARE_EXCHANGE,
            "__wasip1_vfs_host_own_memory_size_compare_exchange"
        );
    }

    #[test]
    fn parses_target_own_memory_size_exports() {
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_own_memory_size_get",
                TARGET_OWN_MEMORY_SIZE_GET_SUFFIX,
            ),
            Some("target_wasm")
        );
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_own_memory_size_set",
                TARGET_OWN_MEMORY_SIZE_SET_SUFFIX,
            ),
            Some("target_wasm")
        );
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_own_memory_size_init",
                TARGET_OWN_MEMORY_SIZE_INIT_SUFFIX,
            ),
            Some("target_wasm")
        );
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_own_memory_size_compare_exchange",
                TARGET_OWN_MEMORY_SIZE_COMPARE_EXCHANGE_SUFFIX,
            ),
            Some("target_wasm")
        );
        assert_eq!(
            parse_prefixed_target_export(
                "target_wasm_own_memory_size_get",
                TARGET_OWN_MEMORY_SIZE_GET_SUFFIX
            ),
            None
        );
    }

    #[test]
    fn parses_memory_copy_exports() {
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_memory_copy_from",
                TARGET_MEMORY_COPY_FROM_SUFFIX,
            ),
            Some("target_wasm")
        );
        assert_eq!(
            parse_prefixed_target_export(
                "__wasip1_vfs_target_wasm_memory_copy_to",
                TARGET_MEMORY_COPY_TO_SUFFIX,
            ),
            Some("target_wasm")
        );
    }
}
