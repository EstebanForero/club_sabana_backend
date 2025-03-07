struct Request {
    requester_id: String,
    request_id: String,
    requested_command: String,
    justification: String,
    approver_id: Option<String>,
}
