# \ActivitiesApi

All URIs are relative to *https://www.strava.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_activity**](ActivitiesApi.md#create_activity) | **POST** /activities | Create an Activity
[**get_activity_by_id**](ActivitiesApi.md#get_activity_by_id) | **GET** /activities/{id} | Get Activity
[**get_comments_by_activity_id**](ActivitiesApi.md#get_comments_by_activity_id) | **GET** /activities/{id}/comments | List Activity Comments
[**get_kudoers_by_activity_id**](ActivitiesApi.md#get_kudoers_by_activity_id) | **GET** /activities/{id}/kudos | List Activity Kudoers
[**get_laps_by_activity_id**](ActivitiesApi.md#get_laps_by_activity_id) | **GET** /activities/{id}/laps | List Activity Laps
[**get_logged_in_athlete_activities**](ActivitiesApi.md#get_logged_in_athlete_activities) | **GET** /athlete/activities | List Athlete Activities
[**get_zones_by_activity_id**](ActivitiesApi.md#get_zones_by_activity_id) | **GET** /activities/{id}/zones | Get Activity Zones
[**update_activity_by_id**](ActivitiesApi.md#update_activity_by_id) | **PUT** /activities/{id} | Update Activity



## create_activity

> models::DetailedActivity create_activity(name, sport_type, start_date_local, elapsed_time, r#type, description, distance, trainer, commute)
Create an Activity

Creates a manual activity for an athlete, requires activity:write scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the activity. | [required] |
**sport_type** | **String** | Sport type of activity. For example - Run, MountainBikeRide, Ride, etc. | [required] |
**start_date_local** | **String** | ISO 8601 formatted date time. | [required] |
**elapsed_time** | **i32** | In seconds. | [required] |
**r#type** | Option<**String**> | Type of activity. For example - Run, Ride etc. |  |
**description** | Option<**String**> | Description of the activity. |  |
**distance** | Option<**f32**> | In meters. |  |
**trainer** | Option<**i32**> | Set to 1 to mark as a trainer activity. |  |
**commute** | Option<**i32**> | Set to 1 to mark as commute. |  |

### Return type

[**models::DetailedActivity**](DetailedActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_by_id

> models::DetailedActivity get_activity_by_id(id, include_all_efforts)
Get Activity

Returns the given activity that is owned by the authenticated athlete. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |
**include_all_efforts** | Option<**bool**> | To include all segments efforts. |  |

### Return type

[**models::DetailedActivity**](DetailedActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_by_activity_id

> Vec<models::Comment> get_comments_by_activity_id(id, page, per_page, page_size, after_cursor)
List Activity Comments

Returns the comments on the given activity. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |
**page** | Option<**i32**> | Deprecated. Prefer to use after_cursor. |  |
**per_page** | Option<**i32**> | Deprecated. Prefer to use page_size. |  |[default to 30]
**page_size** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]
**after_cursor** | Option<**String**> | Cursor of the last item in the previous page of results, used to request the subsequent page of results.  When omitted, the first page of results is fetched. |  |

### Return type

[**Vec<models::Comment>**](Comment.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kudoers_by_activity_id

> Vec<models::SummaryAthlete> get_kudoers_by_activity_id(id, page, per_page)
List Activity Kudoers

Returns the athletes who kudoed an activity identified by an identifier. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<models::SummaryAthlete>**](SummaryAthlete.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_laps_by_activity_id

> Vec<models::Lap> get_laps_by_activity_id(id)
List Activity Laps

Returns the laps of an activity identified by an identifier. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |

### Return type

[**Vec<models::Lap>**](Lap.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_in_athlete_activities

> Vec<models::SummaryActivity> get_logged_in_athlete_activities(before, after, page, per_page)
List Athlete Activities

Returns the activities of an athlete for a specific identifier. Requires activity:read. Only Me activities will be filtered out unless requested by a token with activity:read_all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**i32**> | An epoch timestamp to use for filtering activities that have taken place before a certain time. |  |
**after** | Option<**i32**> | An epoch timestamp to use for filtering activities that have taken place after a certain time. |  |
**page** | Option<**i32**> | Page number. Defaults to 1. |  |
**per_page** | Option<**i32**> | Number of items per page. Defaults to 30. |  |[default to 30]

### Return type

[**Vec<models::SummaryActivity>**](SummaryActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zones_by_activity_id

> Vec<models::ActivityZone> get_zones_by_activity_id(id)
Get Activity Zones

Summit Feature. Returns the zones of a given activity. Requires activity:read for Everyone and Followers activities. Requires activity:read_all for Only Me activities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |

### Return type

[**Vec<models::ActivityZone>**](ActivityZone.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity_by_id

> models::DetailedActivity update_activity_by_id(id, body)
Update Activity

Updates the given activity that is owned by the authenticated athlete. Requires activity:write. Also requires activity:read_all in order to update Only Me activities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The identifier of the activity. | [required] |
**body** | Option<[**UpdatableActivity**](UpdatableActivity.md)> |  |  |

### Return type

[**models::DetailedActivity**](DetailedActivity.md)

### Authorization

[strava_oauth](../README.md#strava_oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

