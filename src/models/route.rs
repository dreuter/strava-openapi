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
pub struct Route {
    #[serde(rename = "athlete", skip_serializing_if = "Option::is_none")]
    pub athlete: Option<Box<models::SummaryAthlete>>,
    /// The description of the route
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The route's distance, in meters
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    /// The route's elevation gain.
    #[serde(rename = "elevation_gain", skip_serializing_if = "Option::is_none")]
    pub elevation_gain: Option<f32>,
    /// The unique identifier of this route
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The unique identifier of the route in string format
    #[serde(rename = "id_str", skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,
    #[serde(rename = "map", skip_serializing_if = "Option::is_none")]
    pub map: Option<Box<models::PolylineMap>>,
    /// The name of this route
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this route is private
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// Whether this route is starred by the logged-in athlete
    #[serde(rename = "starred", skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    /// An epoch timestamp of when the route was created
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// This route's type (1 for ride, 2 for runs)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// This route's sub-type (1 for road, 2 for mountain bike, 3 for cross, 4 for trail, 5 for mixed)
    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<i32>,
    /// The time at which the route was created
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time at which the route was last updated
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Estimated time in seconds for the authenticated athlete to complete route
    #[serde(rename = "estimated_moving_time", skip_serializing_if = "Option::is_none")]
    pub estimated_moving_time: Option<i32>,
    /// The segments traversed by this route
    #[serde(rename = "segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<models::SummarySegment>>,
    /// The custom waypoints along this route
    #[serde(rename = "waypoints", skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<Vec<models::Waypoint>>,
}

impl Route {
    pub fn new() -> Route {
        Route {
            athlete: None,
            description: None,
            distance: None,
            elevation_gain: None,
            id: None,
            id_str: None,
            map: None,
            name: None,
            private: None,
            starred: None,
            timestamp: None,
            r#type: None,
            sub_type: None,
            created_at: None,
            updated_at: None,
            estimated_moving_time: None,
            segments: None,
            waypoints: None,
        }
    }
}

