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
pub struct SummaryClub {
    /// The club's unique identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Resource state, indicates level of detail. Possible values: 1 -> \"meta\", 2 -> \"summary\", 3 -> \"detail\"
    #[serde(rename = "resource_state", skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<i32>,
    /// The club's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL to a 60x60 pixel profile picture.
    #[serde(rename = "profile_medium", skip_serializing_if = "Option::is_none")]
    pub profile_medium: Option<String>,
    /// URL to a ~1185x580 pixel cover photo.
    #[serde(rename = "cover_photo", skip_serializing_if = "Option::is_none")]
    pub cover_photo: Option<String>,
    /// URL to a ~360x176  pixel cover photo.
    #[serde(rename = "cover_photo_small", skip_serializing_if = "Option::is_none")]
    pub cover_photo_small: Option<String>,
    /// Deprecated. Prefer to use activity_types.
    #[serde(rename = "sport_type", skip_serializing_if = "Option::is_none")]
    pub sport_type: Option<SportType>,
    /// The activity types that count for a club. This takes precedence over sport_type.
    #[serde(rename = "activity_types", skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<Vec<models::ActivityType>>,
    /// The club's city.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The club's state or geographical region.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The club's country.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Whether the club is private.
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    /// The club's member count.
    #[serde(rename = "member_count", skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    /// Whether the club is featured or not.
    #[serde(rename = "featured", skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    /// Whether the club is verified or not.
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The club's vanity URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SummaryClub {
    pub fn new() -> SummaryClub {
        SummaryClub {
            id: None,
            resource_state: None,
            name: None,
            profile_medium: None,
            cover_photo: None,
            cover_photo_small: None,
            sport_type: None,
            activity_types: None,
            city: None,
            state: None,
            country: None,
            private: None,
            member_count: None,
            featured: None,
            verified: None,
            url: None,
        }
    }
}
/// Deprecated. Prefer to use activity_types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SportType {
    #[serde(rename = "cycling")]
    Cycling,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "triathlon")]
    Triathlon,
    #[serde(rename = "other")]
    Other,
}

impl Default for SportType {
    fn default() -> SportType {
        Self::Cycling
    }
}

