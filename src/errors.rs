type Message = String;
type Detail = String;


#[derive(Serialize, Debug, Clone)]
#[allow(dead_code, non_camel_case_types)]
pub enum ErrorType {
    BLOB_UNKNOWN,
    BLOB_UPLOAD_INVALID,
    BLOB_UPLOAD_UNKNOWN,
    DIGEST_INVALID,
    MANIFEST_BLOB_UNKNOWN,
    MANIFEST_INVALID,
    MANIFEST_UNKNOWN,
    MANIFEST_UNVERIFIED,
    NAME_INVALID,
    NAME_UNKNOWN,
    SIZE_INVALID,
    TAG_INVALID,
    UNAUTHORIZED,
    DENIED,
    UNSUPPORTED,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Error {
    Error {
        code: ErrorType,
        message: Message,
        detail: Detail,
    }
}

#[derive(Serialize, Debug)]
pub struct Errors {
    errors: Vec<Error>
}

fn get_message(error: &ErrorType) -> Message {
    let message = match *error {
        ErrorType::BLOB_UNKNOWN => "blob unknown to registry",
        ErrorType::BLOB_UPLOAD_INVALID => "blob upload invalid",
        ErrorType::BLOB_UPLOAD_UNKNOWN => "blob upload unknown to registry",
        ErrorType::DIGEST_INVALID => "provided digest did not match uploaded content",
        ErrorType::MANIFEST_BLOB_UNKNOWN => "blob unknown to registry",
        ErrorType::MANIFEST_INVALID => "manifest invalid",
        ErrorType::MANIFEST_UNKNOWN => "manifest unknown",
        ErrorType::MANIFEST_UNVERIFIED => "manifest failed signature verification",
        ErrorType::NAME_INVALID => "invalid repository name",
        ErrorType::NAME_UNKNOWN => "repository not known to registry",
        ErrorType::SIZE_INVALID => "provided length did not match content length",
        ErrorType::TAG_INVALID => "manifest tag did not match URI",
        ErrorType::UNAUTHORIZED => "authentication required",
        ErrorType::DENIED => "requested access to the resource is denied",
        ErrorType::UNSUPPORTED => "The operation is unsupported",
    };
    Message::from(message)
}

fn get_detail(error: &ErrorType) -> Detail {
    let detail = match *error {
        ErrorType::BLOB_UNKNOWN => "This error may be returned when a blob is unknown to the registry in a specified repository. This can be returned with a standard get or if a manifest references an unknown layer during upload",
        ErrorType::BLOB_UPLOAD_INVALID => "The blob upload encountered an error and can no longer proceed",
        ErrorType::BLOB_UPLOAD_UNKNOWN => "If a blob upload has been cancelled or was never started, this error code may be returned",
        ErrorType::DIGEST_INVALID => "When a blob is uploaded, the registry will check that the content matches the digest provided by the client. The error may include a detail structure with the key \"digest\" including the invalid digest string. This error may also be returned when a minfest includes an invalid layer digest.",
        ErrorType::MANIFEST_BLOB_UNKNOWN => "This error may be returned when a manifest blob is unknown to the registry",
        ErrorType::MANIFEST_INVALID => "During upload, manifests undergo several checks ensuring validity. If those checks fail, this error may be returned, unless a more specific error is included. The detail will contain information the failed validation.",
        ErrorType::MANIFEST_UNKNOWN => "This error is returned when the manifest, identified by name and tag is unknown to the repository.",
        ErrorType::MANIFEST_UNVERIFIED => "During manifest upload, if the manifest fails signature verification, this error will be returned.",
        ErrorType::NAME_INVALID => "Invalid repository name encountered either during manifest validation or any API operation.",
        ErrorType::NAME_UNKNOWN => "This is returned if the name used during an operation is unknown to the registry.",
        ErrorType::SIZE_INVALID => "When a layer is uploaded, the provided size will be checked against the uploaded content. If they do not match, this error will be returned.",
        ErrorType::TAG_INVALID => "During a manifest upload, if the tag in the manifest does not match the uri tag, this error will be returned.",
        ErrorType::UNAUTHORIZED => "The access controller was unable to authenticate the client. Often this will be accompanied by a Www-Authenticate HTTP response header indicating how to authenticate.",
        ErrorType::DENIED => "The access controller denied access for the operation on a resource.",
        ErrorType::UNSUPPORTED => "The operation was unsupported due to a missing implementation or invalid set of parameters.",
    };
    Detail::from(detail)
}

pub fn get_error(error: ErrorType) -> Error {
    let message = get_message(&error);
    let detail = get_detail(&error);
    Error::Error{
        code: error,
        message: message,
        detail: detail
    }
}

pub fn generate_errors(errors: &[ErrorType]) -> Errors {
    let mut my_errors: Vec<Error> = Vec::new();
    for error in errors {
        let error = error.clone();
        let error = get_error(error);
        println!("{:?}", error);
        my_errors.push(error);
    }
    Errors {
        errors: my_errors
    }
}

// pub fn generate_errors(errors: &[ErrorType]) -> &Errors {
//     let errors = errors.into_iter().map(|&x| { get_error(x); });
//     let errors: Vec<Error> = Vec::from_iter(errors);
//     &Errors {
//         errors: errors
//     }
// }
