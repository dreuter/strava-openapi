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

/// ActivityTotal : A roll-up of metrics pertaining to a set of activities. Values are in seconds and meters.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityTotal {
    /// The number of activities considered in this total.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The total distance covered by the considered activities.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    /// The total moving time of the considered activities.
    #[serde(rename = "moving_time", skip_serializing_if = "Option::is_none")]
    pub moving_time: Option<i32>,
    /// The total elapsed time of the considered activities.
    #[serde(rename = "elapsed_time", skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<i32>,
    /// The total elevation gain of the considered activities.
    #[serde(rename = "elevation_gain", skip_serializing_if = "Option::is_none")]
    pub elevation_gain: Option<f32>,
    /// The total number of achievements of the considered activities.
    #[serde(rename = "achievement_count", skip_serializing_if = "Option::is_none")]
    pub achievement_count: Option<i32>,
}

impl ActivityTotal {
    /// A roll-up of metrics pertaining to a set of activities. Values are in seconds and meters.
    pub fn new() -> ActivityTotal {
        ActivityTotal {
            count: None,
            distance: None,
            moving_time: None,
            elapsed_time: None,
            elevation_gain: None,
            achievement_count: None,
        }
    }
}

