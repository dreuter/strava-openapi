# SummaryActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique identifier of the activity | [optional]
**external_id** | Option<**String**> | The identifier provided at upload time | [optional]
**upload_id** | Option<**i64**> | The identifier of the upload that resulted in this activity | [optional]
**athlete** | Option<[**models::MetaAthlete**](MetaAthlete.md)> |  | [optional]
**name** | Option<**String**> | The name of the activity | [optional]
**distance** | Option<**f32**> | The activity's distance, in meters | [optional]
**moving_time** | Option<**i32**> | The activity's moving time, in seconds | [optional]
**elapsed_time** | Option<**i32**> | The activity's elapsed time, in seconds | [optional]
**total_elevation_gain** | Option<**f32**> | The activity's total elevation gain. | [optional]
**elev_high** | Option<**f32**> | The activity's highest elevation, in meters | [optional]
**elev_low** | Option<**f32**> | The activity's lowest elevation, in meters | [optional]
**r#type** | Option<[**models::ActivityType**](ActivityType.md)> |  | [optional]
**sport_type** | Option<[**models::SportType**](SportType.md)> |  | [optional]
**start_date** | Option<**String**> | The time at which the activity was started. | [optional]
**start_date_local** | Option<**String**> | The time at which the activity was started in the local timezone. | [optional]
**timezone** | Option<**String**> | The timezone of the activity | [optional]
**start_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**end_latlng** | Option<**Vec<f32>**> | A pair of latitude/longitude coordinates, represented as an array of 2 floating point numbers. | [optional]
**achievement_count** | Option<**i32**> | The number of achievements gained during this activity | [optional]
**kudos_count** | Option<**i32**> | The number of kudos given for this activity | [optional]
**comment_count** | Option<**i32**> | The number of comments for this activity | [optional]
**athlete_count** | Option<**i32**> | The number of athletes for taking part in a group activity | [optional]
**photo_count** | Option<**i32**> | The number of Instagram photos for this activity | [optional]
**total_photo_count** | Option<**i32**> | The number of Instagram and Strava photos for this activity | [optional]
**map** | Option<[**models::PolylineMap**](PolylineMap.md)> |  | [optional]
**trainer** | Option<**bool**> | Whether this activity was recorded on a training machine | [optional]
**commute** | Option<**bool**> | Whether this activity is a commute | [optional]
**manual** | Option<**bool**> | Whether this activity was created manually | [optional]
**private** | Option<**bool**> | Whether this activity is private | [optional]
**flagged** | Option<**bool**> | Whether this activity is flagged | [optional]
**workout_type** | Option<**i32**> | The activity's workout type | [optional]
**upload_id_str** | Option<**String**> | The unique identifier of the upload in string format | [optional]
**average_speed** | Option<**f32**> | The activity's average speed, in meters per second | [optional]
**max_speed** | Option<**f32**> | The activity's max speed, in meters per second | [optional]
**has_kudoed** | Option<**bool**> | Whether the logged-in athlete has kudoed this activity | [optional]
**hide_from_home** | Option<**bool**> | Whether the activity is muted | [optional]
**gear_id** | Option<**String**> | The id of the gear for the activity | [optional]
**kilojoules** | Option<**f32**> | The total work done in kilojoules during this activity. Rides only | [optional]
**average_watts** | Option<**f32**> | Average power output in watts during this activity. Rides only | [optional]
**device_watts** | Option<**bool**> | Whether the watts are from a power meter, false if estimated | [optional]
**max_watts** | Option<**i32**> | Rides with power meter data only | [optional]
**weighted_average_watts** | Option<**i32**> | Similar to Normalized Power. Rides with power meter data only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


