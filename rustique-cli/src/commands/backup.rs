use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{CellAlignment, Color, ContentArrangement, Row, Table};
use rustique_core::api::api_structs::Release;
use rustique_core::api::client::ApiClient;
use crate::commands::arg_structs::info_args::ModInfoArgs;
use rustique_core::config::config_structs::{CellAttr, CellColor};
use rustique_core::information_utils::{notice, prep_cell};
use rustique_core::rustique_errors::RustiqueError;
use rustique_core::utils::html_parse;

pub async fn backup() {

}
