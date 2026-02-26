# Runbook Steps (RT-003)

1. Start local S3-like endpoint on `127.0.0.1:8443`.
2. Execute upload precheck using curl PUT `/bucket/object`.
3. Trigger governed runtime egress request representing uploader flow.
4. Verify deny with `EGRESS_DEST_NOT_CONTRACTED`.
5. Collect standard evidence bundle.
