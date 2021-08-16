use anyhow::Result;

use crate::Client;

pub struct RequestLogs {
    client: Client,
}

impl RequestLogs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RequestLogs { client }
    }

    /**
     * Gets the API request logging log files.
     *
     * This function performs a `GET` to the `/v2.1/diagnostics/request_logs` endpoint.
     *
     * Retrieves a list of log entries as a JSON or xml object or as a zip file containing the entries.
     *
     * If the Accept header is set to application/zip, the response is a zip file containing individual text files, each representing an API request.
     *
     * If the Accept header is set to `application/json` or `application/xml`, the response returns list of log entries in either JSON or XML. An example JSON response body is shown below.
     *
     * **Parameters:**
     *
     * * `encoding: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn api_request_log_get_log(
        &self,
        encoding: &str,
    ) -> Result<crate::types::ApiRequestLogsResult> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !encoding.is_empty() {
            query_args.push(format!("encoding={}", encoding));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/v2.1/diagnostics/request_logs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Deletes the request log files.
     *
     * This function performs a `DELETE` to the `/v2.1/diagnostics/request_logs` endpoint.
     *
     * Deletes the request log files.
     */
    pub async fn api_request_log_delete_logs(&self) -> Result<()> {
        let url = "/v2.1/diagnostics/request_logs".to_string();
        self.client.delete(&url, None).await
    }

    /**
     * Gets a request logging log file.
     *
     * This function performs a `GET` to the `/v2.1/diagnostics/request_logs/{requestLogId}` endpoint.
     *
     * Retrieves information for a single log entry.
     *
     * **Request**
     * The `requestLogfId` property can be retrieved by getting the list of log entries. The Content-Transfer-Encoding header can be set to base64 to retrieve the API request/response as base 64 string. Otherwise the bytes of the request/response are returned.
     *
     * **Response**
     * If the Content-Transfer-Encoding header was set to base64, the log is returned as a base64 string.
     *
     * **Parameters:**
     *
     * * `request_log_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn api_request_log_get(&self, request_log_id: &str) -> Result<Vec<u8>> {
        let url = format!(
            "/v2.1/diagnostics/request_logs/{}",
            crate::progenitor_support::encode_path(&request_log_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Gets a request logging log file.
     *
     * This function performs a `GET` to the `/v2.1/diagnostics/request_logs/{requestLogId}` endpoint.
     *
     * As opposed to `api_request_log_get`, this function returns all the pages of the request at once.
     *
     * Retrieves information for a single log entry.
     *
     * **Request**
     * The `requestLogfId` property can be retrieved by getting the list of log entries. The Content-Transfer-Encoding header can be set to base64 to retrieve the API request/response as base 64 string. Otherwise the bytes of the request/response are returned.
     *
     * **Response**
     * If the Content-Transfer-Encoding header was set to base64, the log is returned as a base64 string.
     */
    pub async fn api_request_log_get_request_logs(&self, request_log_id: &str) -> Result<Vec<u8>> {
        let url = format!(
            "/v2.1/diagnostics/request_logs/{}",
            crate::progenitor_support::encode_path(&request_log_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Gets the API request logging settings.
     *
     * This function performs a `GET` to the `/v2.1/diagnostics/settings` endpoint.
     *
     * Retrieves the current API request logging setting for the user and remaining log entries.
     *
     * **Response**
     * The response includes the current API request logging setting for the user, along with the maximum log entries and remaining log entries.
     */
    pub async fn api_request_log_get_setting(
        &self,
    ) -> Result<crate::types::DiagnosticsSettingsInformation> {
        let url = "/v2.1/diagnostics/settings".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Enables or disables API request logging for troubleshooting.
     *
     * This function performs a `PUT` to the `/v2.1/diagnostics/settings` endpoint.
     *
     * Enables or disables API request logging for troubleshooting.
     *
     * When enabled (`apiRequestLogging` is **true**), REST API requests and responses for the user are added to a log. A log can have up to 50 requests/responses and the current number of log entries can be determined by getting the settings. Logging is automatically disabled when the log limit of 50 is reached.
     *
     * You can call [Diagnostics::getRequestLog
     * ](https://developers.docusign.com/esign-rest-api/reference/Diagnostics/RequestLogs/get) or [Diagnostics::listRequestLogs](https://developers.docusign.com/esign-rest-api/reference/Diagnostics/RequestLogs/list) to download the log files (individually or as a zip file). Call [Diagnostics::deleteRequestLogs
     * ](https://developers.docusign.com/esign-rest-api/reference/Diagnostics/RequestLogs/delete) to clear the log by deleting current entries.
     *
     * Private information, such as passwords and integrator key information, which is normally located in the call header is omitted from the request/response log.
     *
     * API request logging only captures requests from the authenticated user. Any call that does not authenticate the user and resolve a userId is not logged.
     *
     */
    pub async fn api_request_log_put_settings(
        &self,
        body: &crate::types::DiagnosticsSettingsInformation,
    ) -> Result<crate::types::DiagnosticsSettingsInformation> {
        let url = "/v2.1/diagnostics/settings".to_string();
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}