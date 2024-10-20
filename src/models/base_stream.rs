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
pub struct BaseStream {
    /// The number of data points in this stream
    #[serde(rename = "original_size", skip_serializing_if = "Option::is_none")]
    pub original_size: Option<i32>,
    /// The level of detail (sampling) in which this stream was returned
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    /// The base series used in the case the stream was downsampled
    #[serde(rename = "series_type", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<SeriesType>,
}

impl BaseStream {
    pub fn new() -> BaseStream {
        BaseStream {
            original_size: None,
            resolution: None,
            series_type: None,
        }
    }
}
/// The level of detail (sampling) in which this stream was returned
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Resolution {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl Default for Resolution {
    fn default() -> Resolution {
        Self::Low
    }
}
/// The base series used in the case the stream was downsampled
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeriesType {
    #[serde(rename = "distance")]
    Distance,
    #[serde(rename = "time")]
    Time,
}

impl Default for SeriesType {
    fn default() -> SeriesType {
        Self::Distance
    }
}

