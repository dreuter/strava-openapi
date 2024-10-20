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
pub struct DetailedActivity {
    /// The unique identifier of the activity
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The identifier provided at upload time
    #[serde(rename = "external_id", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The identifier of the upload that resulted in this activity
    #[serde(rename = "upload_id", skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<i64>,
    #[serde(rename = "athlete", skip_serializing_if = "Option::is_none")]
    pub athlete: Option<Box<models::MetaAthlete>>,
    /// The name of the activity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The activity's distance, in meters
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    /// The activity's moving time, in seconds
    #[serde(rename = "moving_time", skip_serializing_if = "Option::is_none")]
    pub moving_time: Option<i32>,
    /// The activity's elapsed time, in seconds
    #[serde(rename = "elapsed_time", skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<i32>,
    /// The activity's total elevation gain.
    #[serde(rename = "total_elevation_gain", skip_serializing_if = "Option::is_none")]
    pub total_elevation_gain: Option<f32>,
    /// The activity's highest elevation, in meters
    #[serde(rename = "elev_high", skip_serializing_if = "Option::is_none")]
    pub elev_high: Option<f32>,
    /// The activity's lowest elevation, in meters
    #[serde(rename = "elev_low", skip_serializing_if = "Option::is_none")]
    pub elev_low: Option<f32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::ActivityType>,
    #[serde(rename = "sport_type", skip_serializing_if = "Option::is_none")]
    pub sport_type: Option<models::SportType>,
    /// The time at which the activity was started.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The time at which the activity was started in the local timezone.
    #[serde(rename = "start_date_local", skip_serializing_if = "Option::is_none")]
    pub start_date_local: Option<String>,
    /// The timezone of the activity
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers.
    #[serde(rename = "start_latlng", skip_serializing_if = "Option::is_none")]
    pub start_latlng: Option<Vec<f32>>,
    /// A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers.
    #[serde(rename = "end_latlng", skip_serializing_if = "Option::is_none")]
    pub end_latlng: Option<Vec<f32>>,
    /// The number of achievements gained during this activity
    #[serde(rename = "achievement_count", skip_serializing_if = "Option::is_none")]
    pub achievement_count: Option<i32>,
    /// The number of kudos given for this activity
    #[serde(rename = "kudos_count", skip_serializing_if = "Option::is_none")]
    pub kudos_count: Option<i32>,
    /// The number of comments for this activity
    #[serde(rename = "comment_count", skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i32>,
    /// The number of athletes for taking part in a group activity
    #[serde(rename = "athlete_count", skip_serializing_if = "Option::is_none")]
    pub athlete_count: Option<i32>,
    /// The number of Instagram photos for this activity
    #[serde(rename = "photo_count", skip_serializing_if = "Option::is_none")]
    pub photo_count: Option<i32>,
    /// The number of Instagram and Strava photos for this activity
    #[serde(rename = "total_photo_count", skip_serializing_if = "Option::is_none")]
    pub total_photo_count: Option<i32>,
    #[serde(rename = "map", skip_serializing_if = "Option::is_none")]
    pub map: Option<Box<models::PolylineMap>>,
    /// Whether this activity was recorded on a training machine
    #[serde(rename = "trainer", skip_serializing_if = "Option::is_none")]
    pub trainer: Option<bool>,
    /// Whether this activity is a commute
    #[serde(rename = "commute", skip_serializing_if = "Option::is_none")]
    pub commute: Option<bool>,
    /// Whether this activity was created manually
    #[serde(rename = "manual", skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
    /// Whether this activity is private
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Whether this activity is flagged
    #[serde(rename = "flagged", skip_serializing_if = "Option::is_none")]
    pub flagged: Option<bool>,
    /// The activity's workout type
    #[serde(rename = "workout_type", skip_serializing_if = "Option::is_none")]
    pub workout_type: Option<i32>,
    /// The unique identifier of the upload in string format
    #[serde(rename = "upload_id_str", skip_serializing_if = "Option::is_none")]
    pub upload_id_str: Option<String>,
    /// The activity's average speed, in meters per second
    #[serde(rename = "average_speed", skip_serializing_if = "Option::is_none")]
    pub average_speed: Option<f32>,
    /// The activity's max speed, in meters per second
    #[serde(rename = "max_speed", skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f32>,
    /// Whether the logged-in athlete has kudoed this activity
    #[serde(rename = "has_kudoed", skip_serializing_if = "Option::is_none")]
    pub has_kudoed: Option<bool>,
    /// Whether the activity is muted
    #[serde(rename = "hide_from_home", skip_serializing_if = "Option::is_none")]
    pub hide_from_home: Option<bool>,
    /// The id of the gear for the activity
    #[serde(rename = "gear_id", skip_serializing_if = "Option::is_none")]
    pub gear_id: Option<String>,
    /// The total work done in kilojoules during this activity. Rides only
    #[serde(rename = "kilojoules", skip_serializing_if = "Option::is_none")]
    pub kilojoules: Option<f32>,
    /// Average power output in watts during this activity. Rides only
    #[serde(rename = "average_watts", skip_serializing_if = "Option::is_none")]
    pub average_watts: Option<f32>,
    /// Whether the watts are from a power meter, false if estimated
    #[serde(rename = "device_watts", skip_serializing_if = "Option::is_none")]
    pub device_watts: Option<bool>,
    /// Rides with power meter data only
    #[serde(rename = "max_watts", skip_serializing_if = "Option::is_none")]
    pub max_watts: Option<i32>,
    /// Similar to Normalized Power. Rides with power meter data only
    #[serde(rename = "weighted_average_watts", skip_serializing_if = "Option::is_none")]
    pub weighted_average_watts: Option<i32>,
    /// The description of the activity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "photos", skip_serializing_if = "Option::is_none")]
    pub photos: Option<Box<models::PhotosSummary>>,
    #[serde(rename = "gear", skip_serializing_if = "Option::is_none")]
    pub gear: Option<Box<models::SummaryGear>>,
    /// The number of kilocalories consumed during this activity
    #[serde(rename = "calories", skip_serializing_if = "Option::is_none")]
    pub calories: Option<f32>,
    #[serde(rename = "segment_efforts", skip_serializing_if = "Option::is_none")]
    pub segment_efforts: Option<Vec<models::DetailedSegmentEffort>>,
    /// The name of the device used to record the activity
    #[serde(rename = "device_name", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// The token used to embed a Strava activity
    #[serde(rename = "embed_token", skip_serializing_if = "Option::is_none")]
    pub embed_token: Option<String>,
    /// The splits of this activity in metric units (for runs)
    #[serde(rename = "splits_metric", skip_serializing_if = "Option::is_none")]
    pub splits_metric: Option<Vec<models::Split>>,
    /// The splits of this activity in imperial units (for runs)
    #[serde(rename = "splits_standard", skip_serializing_if = "Option::is_none")]
    pub splits_standard: Option<Vec<models::Split>>,
    #[serde(rename = "laps", skip_serializing_if = "Option::is_none")]
    pub laps: Option<Vec<models::Lap>>,
    #[serde(rename = "best_efforts", skip_serializing_if = "Option::is_none")]
    pub best_efforts: Option<Vec<models::DetailedSegmentEffort>>,
}

impl DetailedActivity {
    pub fn new() -> DetailedActivity {
        DetailedActivity {
            id: None,
            external_id: None,
            upload_id: None,
            athlete: None,
            name: None,
            distance: None,
            moving_time: None,
            elapsed_time: None,
            total_elevation_gain: None,
            elev_high: None,
            elev_low: None,
            r#type: None,
            sport_type: None,
            start_date: None,
            start_date_local: None,
            timezone: None,
            start_latlng: None,
            end_latlng: None,
            achievement_count: None,
            kudos_count: None,
            comment_count: None,
            athlete_count: None,
            photo_count: None,
            total_photo_count: None,
            map: None,
            trainer: None,
            commute: None,
            manual: None,
            private: None,
            flagged: None,
            workout_type: None,
            upload_id_str: None,
            average_speed: None,
            max_speed: None,
            has_kudoed: None,
            hide_from_home: None,
            gear_id: None,
            kilojoules: None,
            average_watts: None,
            device_watts: None,
            max_watts: None,
            weighted_average_watts: None,
            description: None,
            photos: None,
            gear: None,
            calories: None,
            segment_efforts: None,
            device_name: None,
            embed_token: None,
            splits_metric: None,
            splits_standard: None,
            laps: None,
            best_efforts: None,
        }
    }
}

