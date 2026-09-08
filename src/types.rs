//! Strongly-typed domain primitives and unforgeable newtypes.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Strongly typed Email address guaranteeing non-empty and '@' presence at creation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Creates a new validated `Email` instance.
    pub fn new(email: impl Into<String>) -> Result<Self, crate::error::Error> {
        let s = email.into();
        let trimmed = s.trim();
        if trimmed.is_empty() || !trimmed.contains('@') {
            return Err(crate::error::Error::Invariant(format!(
                "Invalid email format: '{trimmed}'"
            )));
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns a string slice of the email address.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes into the inner String.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Email {
    type Err = crate::error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Macro to define zero-cost, type-safe ID newtypes.
#[macro_export]
macro_rules! define_id {
    ($name:ident, $inner:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub $inner);

        impl $name {
            #[doc = "Create a new instance."]
            #[must_use]
            pub const fn new(id: $inner) -> Self {
                Self(id)
            }

            #[doc = "Get the inner value."]
            #[must_use]
            pub const fn get(&self) -> $inner {
                self.0
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$inner> for $name {
            fn from(val: $inner) -> Self {
                Self(val)
            }
        }

        impl From<$name> for $inner {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
    (str $name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            #[doc = "Create a new instance."]
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            #[doc = "Get a string slice."]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            #[doc = "Consume into inner String."]
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<T: Into<String>> From<T> for $name {
            fn from(val: T) -> Self {
                Self(val.into())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

define_id!(ContactId, i64, "Unique identifier for a Contact.");
define_id!(
    CampaignId,
    i64,
    "Unique identifier for a Marketing Campaign."
);
define_id!(
    TemplateId,
    i64,
    "Unique identifier for an Email or SMS Template."
);
define_id!(WebhookId, i64, "Unique identifier for a Webhook.");
define_id!(
    ProcessId,
    i64,
    "Unique identifier for an asynchronous process/import task."
);
define_id!(ListId, i64, "Unique identifier for a Contact List.");
define_id!(FolderId, i64, "Unique identifier for a Folder.");
define_id!(SenderId, i64, "Unique identifier for a Sender.");
define_id!(UserId, i64, "Unique identifier for a User.");

define_id!(str MessageId, "Unique message identifier returned by transactional services.");
define_id!(str DealId, "Unique deal identifier.");
define_id!(str CompanyId, "Unique company identifier.");
define_id!(str TaskId, "Unique CRM task identifier.");
define_id!(str NoteId, "Unique note identifier.");
define_id!(str CouponId, "Unique coupon identifier.");
define_id!(str DomainName, "Domain name wrapper.");
define_id!(str OrganizationId, "Organization identifier.");
