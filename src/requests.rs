//! Types to handle requests.

use std::time::Duration;

use matrix_sdk_common::ruma::{
    api::client::keys::{
        claim_keys::v3::Request as RumaKeysClaimRequest,
        upload_keys::v3::Request as RumaKeysUploadRequest,
        upload_signatures::v3::Request as RumaSignatureUploadRequest,
    },
    events::EventContent,
};
use matrix_sdk_crypto::types::requests::{
    AnyOutgoingRequest, KeysBackupRequest as RumaKeysBackupRequest,
    KeysQueryRequest as RumaKeysQueryRequest, OutgoingRequest as SdkOutgoingRequest,
    RoomMessageRequest as RumaRoomMessageRequest, ToDeviceRequest as RumaToDeviceRequest,
};
use napi::bindgen_prelude::Either6;
use napi_derive::*;

use crate::into_err;

/// Data for a request to the `/keys/upload` API endpoint
/// ([specification]).
///
/// Publishes end-to-end encryption keys for the device.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysupload
#[napi]
pub struct KeysUploadRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A JSON-encoded string containing the rest of the payload: `device_keys`,
    /// `one_time_keys`, `fallback_keys`.
    ///
    /// It represents the body of the HTTP request.
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl KeysUploadRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysUpload
    }
}

/// Data for a request to the `/keys/query` API endpoint
/// ([specification]).
///
/// Returns the current devices and identity keys for the given users.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysquery
#[napi]
pub struct KeysQueryRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"timeout": …, "device_keys": …}
    /// ```
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl KeysQueryRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysQuery
    }
}

/// Data for a request to the `/keys/claim` API endpoint
/// ([specification]).
///
/// Claims one-time keys that can be used to establish 1-to-1 E2EE
/// sessions.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keysclaim
#[napi]
pub struct KeysClaimRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A JSON-encoded object of form:
    ///
    /// ```json
    /// {"timeout": …,  "one_time_keys": …}
    /// ```
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl KeysClaimRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysClaim
    }
}

/// Data for a request to the `/sendToDevice` API endpoint
/// ([specification]).
///
/// Send an event to a single device or to a group of devices.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3sendtodeviceeventtypetxnid
#[napi]
pub struct ToDeviceRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A string representing the type of event being sent to each devices.
    #[napi(readonly)]
    pub event_type: String,

    /// A string representing a request identifier unique to the access token
    /// used to send the request.
    #[napi(readonly)]
    pub txn_id: String,

    /// A JSON-encoded string containing the rest of the payload: `messages`.
    ///
    /// It represents the body of the HTTP request.
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl ToDeviceRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::ToDevice
    }
}

/// Data for a request to the `/keys/signatures/upload` API endpoint
/// ([specification]).
///
/// Publishes cross-signing signatures for the user.
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#post_matrixclientv3keyssignaturesupload
#[napi]
pub struct SignatureUploadRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A JSON-encoded string containing the rest of the payload: `signed_keys`.
    ///
    /// It represents the body of the HTTP request.
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl SignatureUploadRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::SignatureUpload
    }
}

/// A customized owned request type for sending out room messages
/// ([specification]).
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3roomsroomidsendeventtypetxnid
#[napi]
pub struct RoomMessageRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A string representing the room to send the event to.
    #[napi(readonly)]
    pub room_id: String,

    /// A string representing the transaction ID for this event.
    ///
    /// Clients should generate an ID unique across requests with the same
    /// access token; it will be used by the server to ensure idempotency of
    /// requests.
    #[napi(readonly)]
    pub txn_id: String,

    /// A string representing the type of event to be sent.
    #[napi(readonly)]
    pub event_type: String,

    /// A JSON-encoded string containing the message's content.
    #[napi(readonly, js_name = "body")]
    pub content: String,
}

#[napi]
impl RoomMessageRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::RoomMessage
    }
}

/// A request that will back up a batch of room keys to the server
/// ([specification]).
///
/// [specification]: https://spec.matrix.org/unstable/client-server-api/#put_matrixclientv3room_keyskeys
#[napi]
pub struct KeysBackupRequest {
    /// The request ID.
    #[napi(readonly)]
    pub id: String,

    /// A JSON-encoded string containing the rest of the payload: `rooms`.
    ///
    /// It represents the body of the HTTP request.
    #[napi(readonly)]
    pub body: String,
}

#[napi]
impl KeysBackupRequest {
    /// Get its request type.
    #[napi(getter, js_name = "type")]
    pub fn request_type(&self) -> RequestType {
        RequestType::KeysBackup
    }
}

macro_rules! request {
    (
        $destination_request:ident from $source_request:ident
        $( extracts $( $field_name:ident : $field_type:tt ),+ $(,)? )?
        $( $( and )? groups $( $grouped_field_name:ident $( { $grouped_field_transformation:expr } )? ),+ $(,)? )?
    ) => {
        impl TryFrom<&$source_request> for $destination_request {
            type Error = napi::Error;

            fn try_from(request: &$source_request) -> Result<Self, Self::Error> {
                request!(
                    @__try_from $destination_request from $source_request
                    (request_id = String::new(), request = request)
                    $( extracts [ $( $field_name : $field_type, )+ ] )?
                    $( groups [ $( $grouped_field_name $( { $grouped_field_transformation } )? , )+ ] )?
                )
            }
        }

        impl TryFrom<(String, &$source_request)> for $destination_request {
            type Error = napi::Error;

            fn try_from(
                (request_id, request): (String, &$source_request),
            ) -> Result<Self, Self::Error> {
                request!(
                    @__try_from $destination_request from $source_request
                    (request_id = request_id.into(), request = request)
                    $( extracts [ $( $field_name : $field_type, )+ ] )?
                    $( groups [ $( $grouped_field_name $( { $grouped_field_transformation } )? , )+ ] )?
                )
            }
        }
    };

    (
        @__try_from $destination_request:ident from $source_request:ident
        (request_id = $request_id:expr, request = $request:expr)
        $( extracts [ $( $field_name:ident : $field_type:tt ),* $(,)? ] )?
        $( groups [ $( $grouped_field_name:ident $( { $grouped_field_transformation:expr } )? ),* $(,)? ] )?
    ) => {
        {
            Ok($destination_request {
                id: $request_id,
                $(
                    $(
                        $field_name: request!(@__field $field_name : $field_type ; request = $request),
                    )*
                )?
                $(
                    body: {
                        let mut map = serde_json::Map::new();
                        $(

                            let field = &$request.$grouped_field_name;
                            $(
                                let field = {
                                    let $grouped_field_name = field;

                                    $grouped_field_transformation
                                };
                            )?
                            map.insert(stringify!($grouped_field_name).to_owned(), serde_json::to_value(field).map_err(into_err)?);
                        )*
                        let object = serde_json::Value::Object(map);

                        serde_json::to_string(&object).map_err(into_err)?.into()
                    }
                )?
            })
        }
    };

    ( @__field $field_name:ident : $field_type:ident ; request = $request:expr ) => {
        request!(@__field_type as $field_type ; request = $request, field_name = $field_name)
    };

    ( @__field_type as string ; request = $request:expr, field_name = $field_name:ident ) => {
        $request.$field_name.to_string().into()
    };

    ( @__field_type as json ; request = $request:expr, field_name = $field_name:ident ) => {
        serde_json::to_string(&$request.$field_name).map_err(into_err)?.into()
    };

    ( @__field_type as event_type ; request = $request:expr, field_name = $field_name:ident ) => {
        $request.content.event_type().to_string().into()
    };
}

request!(KeysUploadRequest from RumaKeysUploadRequest groups device_keys, one_time_keys, fallback_keys);
request!(KeysQueryRequest from RumaKeysQueryRequest groups timeout { timeout.as_ref().map(Duration::as_millis).map(u64::try_from).transpose().map_err(into_err)? }, device_keys);
request!(KeysClaimRequest from RumaKeysClaimRequest groups timeout { timeout.as_ref().map(Duration::as_millis).map(u64::try_from).transpose().map_err(into_err)? }, one_time_keys);
request!(ToDeviceRequest from RumaToDeviceRequest extracts event_type: string, txn_id: string and groups messages);
request!(SignatureUploadRequest from RumaSignatureUploadRequest groups signed_keys);
request!(RoomMessageRequest from RumaRoomMessageRequest extracts room_id: string, txn_id: string, event_type: event_type, content: json);
request!(KeysBackupRequest from RumaKeysBackupRequest groups rooms);

pub type OutgoingRequests = Either6<
    KeysUploadRequest,
    KeysQueryRequest,
    KeysClaimRequest,
    ToDeviceRequest,
    SignatureUploadRequest,
    RoomMessageRequest,
>;

pub(crate) struct OutgoingRequest(pub(crate) SdkOutgoingRequest);

impl TryFrom<OutgoingRequest> for OutgoingRequests {
    type Error = napi::Error;

    fn try_from(outgoing_request: OutgoingRequest) -> Result<Self, Self::Error> {
        let request_id = outgoing_request.0.request_id().to_string();

        Ok(match outgoing_request.0.request() {
            AnyOutgoingRequest::KeysUpload(request) => {
                Either6::A(KeysUploadRequest::try_from((request_id, request))?)
            }

            AnyOutgoingRequest::KeysQuery(request) => {
                Either6::B(KeysQueryRequest::try_from((request_id, request))?)
            }

            AnyOutgoingRequest::KeysClaim(request) => {
                Either6::C(KeysClaimRequest::try_from((request_id, request))?)
            }

            AnyOutgoingRequest::ToDeviceRequest(request) => {
                Either6::D(ToDeviceRequest::try_from((request_id, request))?)
            }

            AnyOutgoingRequest::SignatureUpload(request) => {
                Either6::E(SignatureUploadRequest::try_from((request_id, request))?)
            }

            AnyOutgoingRequest::RoomMessage(request) => {
                Either6::F(RoomMessageRequest::try_from((request_id, request))?)
            }
        })
    }
}

/// Represent the type of a request.
#[napi]
pub enum RequestType {
    /// Represents a `KeysUploadRequest`.
    KeysUpload,

    /// Represents a `KeysQueryRequest`.
    KeysQuery,

    /// Represents a `KeysClaimRequest`.
    KeysClaim,

    /// Represents a `ToDeviceRequest`.
    ToDevice,

    /// Represents a `SignatureUploadRequest`.
    SignatureUpload,

    /// Represents a `RoomMessageRequest`.
    RoomMessage,

    /// Represents a `KeysBackupRequest`.
    KeysBackup,
}
