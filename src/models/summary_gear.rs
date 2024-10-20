/*
 * Strava API v3
 *
 * The [Swagger Playground](https://developers.strava.com/playground) is the easiest way to familiarize yourself with the Strava API by submitting HTTP requests and observing the responses before you write any client code. It will show what a response will look like with different endpoints depending on the authorization scope you receive from your athletes. To use the Playground, go to https://www.strava.com/settings/api and change your “Authorization Callback Domain” to developers.strava.com. Please note, we only support Swagger 2.0. There is a known issue where you can only select one scope at a time. For more information, please check the section “client code” at https://developers.strava.com/docs.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryGear {
    /// The gear's unique identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource state, indicates level of detail. Possible values: 2 -> \"summary\", 3 -> \"detail\"
    #[serde(rename = "resource_state", skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<i32>,
    /// Whether this gear's is the owner's default one.
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// The gear's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The distance logged with this gear.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
}

impl SummaryGear {
    pub fn new() -> SummaryGear {
        SummaryGear {
            id: None,
            resource_state: None,
            primary: None,
            name: None,
            distance: None,
        }
    }
}

