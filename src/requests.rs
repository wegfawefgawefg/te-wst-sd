enum SubmissionFailure {
    BigSubmissionAccepted,
    SmallSubmissionAccepted,
    TooBigSubmissionRejected,
    MaliciousSubmissionRejected,
    TooManySubmissionsRejected,
}

#[get("/new_submisison/<user_id>/<file_size_in_bytes>")]
pub fn new_submission(
    user_id: UserId,
    file_size_in_bytes: FileSizeInBytes,
) -> Result<SubmissionId, SubmissionFailure> {
    //// Some Guard Clauses ////
    // check malicious
    // check user valid
    // check user cooldown
    // check max file size

    if file_size_in_bytes > TOO_BIG_THRESHOLD {
        return SubmissionResult::TooBigSubmissionRejected;
    }

    if file_size_in_bytes > BIG_SMALL_THRESHOLD {
        handle_big_submission(user_id, file_size_in_bytes);
    } else {
        handle_small_submission(user_id, file_size_in_bytes);
    }
}

#[get("/get_available_submissions/<user_id>")]
pub fn get_available_submissions(user_id: UserId) -> Vec<SubmissionId> {
    // get available submissions from db
}

enum SocketRequestFailure {
    SubmissionIdDoesNotExist,
    MaliciousRequest,
    TooManyRequests,
}

// request for getting a websocket connection for big file streaming
#[get("/big_file_streaming/<user_id>/<submission_id>")]
pub fn get_big_file_streaming_socket(
    user_id: UserId,
    submission_id: SubmissionId,
) -> Result<WebSocket, SocketRequestFailure> {
    ////// guard clauses
    // get websocket connection

    return Result::Err(SocketRequestFailure::SubmissionIdDoesNotExist);
}
