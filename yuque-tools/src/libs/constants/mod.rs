/*
 * Description: 配置文件
 * Created: 2023-08-31 19:20:57
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */
use lazy_static::lazy_static;
pub mod load_config;
pub mod schema;
use crate::libs::constants::load_config::{load_conf, GlobalConfig};

lazy_static! {
    pub static ref GLOBAL_CONFIG: GlobalConfig = load_conf();
}
