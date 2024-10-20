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
pub struct PolylineMap {
    /// The identifier of the map
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The polyline of the map, only returned on detailed representation of an object
    #[serde(rename = "polyline", skip_serializing_if = "Option::is_none")]
    pub polyline: Option<String>,
    /// The summary polyline of the map
    #[serde(rename = "summary_polyline", skip_serializing_if = "Option::is_none")]
    pub summary_polyline: Option<String>,
}

impl PolylineMap {
    pub fn new() -> PolylineMap {
        PolylineMap {
            id: None,
            polyline: None,
            summary_polyline: None,
        }
    }
}

