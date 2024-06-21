use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use uuid::{uuid, Uuid};

use crate::{
    data::{Bytes, Version},
    suite::AsymmetricCipherAlgorithm,
};

pub const PROTOCOL_ID_PASSMAN: Uuid = uuid!("019038bd-15b8-75b5-8de3-9e6dfd801916");

pub const PROTOCOL_VERSION: Version = Version::from_parts(0, 1, 0);

/// `GET /hello`
///
/// Returns information about the server
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Hello {
    /// Returns the unique ID of the server.
    pub server_id: Uuid,
    /// Returns the unique ID of the protocol supported by the server
    pub protocol_id: Uuid,
    /// The Time at which the `/hello` request was serviced.
    pub hello_time: PrimitiveDateTime,
}

pub mod acl {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    #[serde(rename_all = "kebab-case")]
    pub enum AclMode {
        Inherit,
        Allow,
        Deny,
        Forbid,
    }

    /// ## Get All ACL Rows for an object
    ///
    /// `GET /<object-type>/<uuid>/acl`
    ///
    /// Where object-type is either `item` or `user`.
    ///
    /// Requires: ACL Permission `ReadAcl` on `uuid`.
    ///
    /// ## Get All ACL Rows for an object associated with a given user
    ///
    /// `GET /<object-type>/<object-uuid>/acl?subject=<subject-uuid>`
    /// Where object-type is either `item` or `user`.
    ///
    /// Requires: ACL Permission `ReadAcl` on `uuid`.
    ///
    /// ## Create or modify a new ACL Row or Rows
    ///
    /// `POST /<object-type>/<uuid>/acl`
    /// Where object-type is either `item` or `user`.
    ///
    /// Row is checked by subject id and action. Identical rows are replaced.
    ///
    /// Requires: ACL Permission `WriteAcl` on `uuid`.
    ///
    /// Modifying the `Owner` action requires having the `Owner` permission or the `TakeOwnership` global permission
    ///
    /// ## Replace Entire Object ACL
    ///
    /// `PUT /<object-type>/<uuid>/acl` Array of [`AclRow`]
    ///  Where object-type is either `item` or `user`.
    ///
    /// Requires: ACL Permission `Owner` on `uuid`.
    ///
    /// ## Get Global Permission set
    ///
    /// `GET /server/permissions`
    ///
    /// Requires: `ReadAcl` global permission
    ///
    /// ## Update Global Permission set
    ///
    /// `POST /server/permissions`
    ///
    /// Requires: `WriteAcl` global permission.
    ///  Modifying the `Owner` action requires having the `Owner` global permission.
    ///
    /// Same operation as `POST /<object-type>/<uuid>/acl`
    ///
    /// ### Replace Entire Global Permissions
    ///
    /// `PUT /server/permissions` Array of [`AclRow`]
    ///
    /// Requires: `Owner` global permission
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct AclRow {
        pub subject: Uuid,
        pub action: String,
        pub mode: AclMode,
    }
}

/// ## Deleting a User
///
/// `DELETE /user/<uuid>`
///
/// Requires: Authed as user `<uuid>` or server admin
///
pub mod user {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{data::Bytes, suite::AsymmetricCipherAlgorithm};

    use super::auth::UserAuth;

    /// ## Retrieving a User Info
    ///
    /// `GET /users/<uuid>/root`
    ///
    /// Requires: Authenticated as user `<uuid>` or have ACL permission `ReadRootInfo`
    ///
    /// ## Updating User Info
    ///
    /// `PUT /users/<uuid>/root`
    ///
    /// Requires: Authed as user `<uuid>` or have ACL permission `WriteRootInfo`
    ///
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct UserRootInfo {
        pub root_object: Uuid,
        pub root_key: Uuid,
    }

    /// ## Retrieving User Public Key
    ///
    /// `GET /users/<uuid>/public-key`
    ///
    /// Requires: Authenticated.
    ///
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct UserPublicKey {
        pub pub_key: Bytes,
        pub pub_key_alg: AsymmetricCipherAlgorithm,
    }

    /// # Creating a New User
    /// `POST /users/new`
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct NewUserRequest {
        pub user_address: String,
        pub initial_auth: UserAuth,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct NewUserResponse {
        pub user_id: Uuid,
    }
}

pub mod auth {
    use crate::{
        data::Bytes,
        suite::{AsymmetricCipherAlgorithm, DigestAlgorithm},
    };
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    /// ## Retreiving auth info
    ///
    /// `GET /users/<uuid>/auth`
    ///
    /// ## Updating auth info
    ///
    /// `PUT /users/<uuid>/auth`
    ///
    /// Requires: Authenticated as `<uuid>`.
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct UserAuth {
        pub kdf_base_digest_alg: DigestAlgorithm,
        pub auth_key_alg: AsymmetricCipherAlgorithm,
        pub pub_key: Bytes,
        pub priv_key_iv: Bytes,
        pub secured_private_key: Bytes,
    }

    /// # Starting authentication
    ///
    /// `POST /auth/challenge`
    ///
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct AuthChallengeRequest {
        pub user_id: Uuid,
        pub challenge_session_id: Uuid,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct AuthChallengeResponse {
        pub challenge_digest: DigestAlgorithm,
        pub challenge_bytes: Bytes,
    }

    /// # Fulfilling Authentication
    ///
    /// `POST /auth/response`
    ///
    /// `Authorization: Bearer <challenge_session_id>`
    ///
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct AuthResponse {
        pub challenge_signature: Bytes,
    }
}

///
/// ## Retrieve an Item
///
/// `GET /item/<uuid>`
///
/// Accept: application/octet-stream
///
/// Requires: ACL Permission `Read`
///
/// ## Update an Item's Contents
///
/// `PUT /items/<uuid>`
///
/// Content-Type: application/octet-stream
///
/// Requires: ACL Permission `Write`
/// ## Delete an Item
///
/// `DELETE /items/<uuid>`
///
/// Requires: ACL Permission `Delete`.
pub mod item {
    use serde::{Deserialize, Serialize};
    use time::PrimitiveDateTime;
    use uuid::Uuid;

    use crate::data::Bytes;

    use crate::suite::SymmetricCipherAlgorithm;

    /// ## Retrieve Item Key Information for a given key
    ///
    /// `GET /items/<item-uuid>/keys/<key-uuid>`
    ///
    /// Requires: ACL Permission `Read` for `item`.
    ///
    /// ## Update item Key Information for a given key, or add new key to item
    ///
    /// `PUT /items/<item-uuid>/keys/<key-uuid>`
    ///
    /// Requires: ACL Permission `WriteKeys` for `item`.
    ///
    /// ## Remove a Key from an Item
    ///
    /// `DELETE /items/<item-uuid>/keys/<key-uuid>`
    ///
    /// Requires: ACL Permission `DeleteKeys` for `item`.
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct ItemKeyInfo {
        pub secured_item_key: Bytes,
        pub item_key_iv: Bytes,
        pub item_auth_tag: Option<Bytes>,
    }

    /// ## Retrieve Item Key List
    ///
    /// `GET /items/<item-uuid>/keys`
    ///
    /// Requires: ACL Permission `Read` for `item`.
    ///
    /// ## Update Item Key List
    ///
    /// `PUT /items/<item-uuid>/keys`
    ///
    /// Requires: ACL Permission `WriteKeys` for `item`.
    ///
    /// ## Remove all keys from an item
    ///
    /// `DELETE /items/<item-uuid>/keys`
    ///
    /// Requires: ACL Permission `DeleteKeys` for `item`.
    #[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct ItemKeys {
        pub base_cipher: SymmetricCipherAlgorithm,
        pub key_refs: Vec<Uuid>,
        pub item_iv: Bytes,
        pub item_auth_tag: Option<Bytes>,
    }

    /// ## Retrieve Item Metadata
    ///
    /// `GET /items/<item-uuid>/metadata`
    pub struct ItemMetadata {
        pub content_type: String,
        pub mtime: PrimitiveDateTime,
        pub atime: PrimitiveDateTime,
        pub ctime: PrimitiveDateTime,
    }
}
