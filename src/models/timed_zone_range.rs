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

/// TimedZoneRange : A union type representing the time spent in a given zone.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimedZoneRange {
    /// The minimum value in the range.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// The maximum value in the range.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// The number of seconds spent in this zone
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
}

impl TimedZoneRange {
    /// A union type representing the time spent in a given zone.
    pub fn new() -> TimedZoneRange {
        TimedZoneRange {
            min: None,
            max: None,
            time: None,
        }
    }
}

