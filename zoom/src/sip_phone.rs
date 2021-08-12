use anyhow::Result;

use crate::Client;

pub struct SipPhone {
    client: Client,
}

impl SipPhone {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SipPhone { client }
    }

    /**
     * List SIP phones.
     *
     * This function performs a `GET` to the `/sip_phones` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to list SIP phones on an account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * User must enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     * **Parameters:**
     *
     * * `page_number: i64` -- *  \*\*Deprecated\*\* - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *  
     *  The page number of the current page in the returned records.
     * * `search_key: &str` -- User name or email address of a user. If this parameter is provided, only the SIP phone system integration enabled for that specific user will be returned. Otherwise, all SIP phones on an account will be returned.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_sip_phones(
        &self,
        page_number: i64,
        search_key: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::ListSipPhonesResponse> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(format!("next_page_token={}", next_page_token));
        }
        if page_number > 0 {
            query_args.push(format!("page_number={}", page_number));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !search_key.is_empty() {
            query_args.push(format!("search_key={}", search_key));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/sip_phones?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * Enable SIP phone.
     *
     * This function performs a `POST` to the `/sip_phones` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to enable a user to use SIP phone.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * The account owner or account admin must first enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:write:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     */
    pub async fn create_sip_phone(&self, body: &crate::types::CreateSipPhoneRequest) -> Result<()> {
        let url = "/sip_phones".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Delete SIP phone.
     *
     * This function performs a `DELETE` to the `/sip_phones/{phoneId}` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to delete a specific SIP phone on a Zoom account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * User must enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:read:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `phone_id: &str` -- Unique Identifier of the SIP Phone. It can be retrieved from the List SIP Phones API.
     */
    pub async fn delete_sip_phone(&self, phone_id: &str, phone_id: &str) -> Result<()> {
        let url = format!(
            "/sip_phones/{}",
            crate::progenitor_support::encode_path(&phone_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update SIP phone.
     *
     * This function performs a `PATCH` to the `/sip_phones/{phoneId}` endpoint.
     *
     * Zoom’s Phone System Integration (PSI), also referred as SIP phones, enables an organization to leverage the Zoom client to complete a softphone registration to supported premise based PBX system. End users will have the ability to have softphone functionality within a single client while maintaining a comparable interface to Zoom Phone. Use this API to update information of a specific SIP Phone on a Zoom account.<br><br>
     * **Prerequisites**:
     * * Currently only supported on Cisco and Avaya PBX systems.
     * * The account owner or account admin must first enable SIP Phone Integration by contacting the [Sales](https://zoom.us/contactsales) team.<br> **Scope:** `sip_phone:write:admin`
     * <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `phone_id: &str` -- Unique Identifier of the SIP Phone. This can be retrieved from the List SIP Phones API.
     */
    pub async fn update_sip_phone(
        &self,
        phone_id: &str,
        phone_id: &str,
        body: &crate::types::UpdateSipPhoneRequest,
    ) -> Result<()> {
        let url = format!(
            "/sip_phones/{}",
            crate::progenitor_support::encode_path(&phone_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}