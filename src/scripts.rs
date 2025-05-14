use std::{env, fs};

pub fn read_common_sh() -> (String, String, String) {
    const COMMON_SH_PATH: &str = "../common.sh";
    let path = env::current_exe().unwrap().parent().unwrap().join(COMMON_SH_PATH);
    let contents = fs::read_to_string(path).expect("Failed to read common.sh");
    let mut aosp_tag = String::new();
    let mut aosp_tag_old = String::new();
    let mut branch = String::new();

    for line in contents.lines() {
        if line.starts_with("readonly aosp_tag=") {
            aosp_tag = line.trim_start_matches("readonly aosp_tag=").trim_matches('"').to_string();
        } else if line.starts_with("readonly aosp_tag_old=") {
            aosp_tag_old = line.trim_start_matches("readonly aosp_tag_old=").trim_matches('"').to_string();
        } else if line.starts_with("readonly branch=") {
            branch = line.trim_start_matches("readonly branch=").trim_matches('"').to_string();
        }
    }

    (aosp_tag, aosp_tag_old, branch)
}

pub fn read_config_file() -> (String, String, String) {
    const CONFIG_PATH: &str = "common";
    let path = env::current_exe().unwrap().parent().unwrap().join(CONFIG_PATH);
    let contents = fs::read_to_string(path).expect("Failed to read common.sh");
    let mut graphene_tag = String::new();
    let mut graphene_tag_old = String::new();
    let mut lineage_latest_branch = String::new();

    for line in contents.lines() {
        if line.starts_with("readonly graphene_tag=") {
            graphene_tag = line.trim_start_matches("readonly graphene_tag=").trim_matches('"').to_string();
        } else if line.starts_with("readonly graphene_tag_old=") {
            graphene_tag_old = line.trim_start_matches("readonly graphene_tag_old=").trim_matches('"').to_string();
        } else if line.starts_with("readonly lineage_latest_branch=") {
            lineage_latest_branch = line.trim_start_matches("readonly lineage_latest_branch=").trim_matches('"').to_string();
        }
    }

    (graphene_tag, graphene_tag_old, lineage_latest_branch)
}
