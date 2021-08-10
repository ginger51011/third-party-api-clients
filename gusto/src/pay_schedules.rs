use anyhow::Result;

use crate::Client;

pub struct PaySchedules {
    client: Client,
}

impl PaySchedules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PaySchedules { client }
    }

    /**
     * Get the pay schedules for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/pay_schedules` endpoint.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_v_1_companies_company_id_pay_schedules(
        &self,
    ) -> Result<Vec<crate::types::PaySchedule>> {
        let url = format!(
            "/v1/companies/{}/pay_schedules",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get the pay schedules for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/pay_schedules` endpoint.
     *
     * As opposed to `get_v_1_companies_company_id_pay_schedules`, this function returns all the pages of the request at once.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_v_1_companies_company_id_pay_schedules(
        &self,
    ) -> Result<Vec<crate::types::PaySchedule>> {
        let url = format!(
            "/v1/companies/{}/pay_schedules",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Get a pay schedule.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}` endpoint.
     *
     * The pay schedule object in Gusto captures the details of when employees work and when they should be paid. A company can have multiple pay schedules.
     */
    pub async fn get_v_1_companies_company_id_pay_schedules_pay_schedule_id(
        &self,
    ) -> Result<crate::types::PaySchedule> {
        let url = format!(
            "/v1/companies/{}/pay_schedules/{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            crate::progenitor_support::encode_path(&pay_schedule_id_or_uuid.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update a pay schedule.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}` endpoint.
     *
     * Updates a pay schedule.
     *
     * This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto
     */
    pub async fn put_v_1_companies_company_id_pay_schedules_pay_schedule_id(
        &self,
        body: &crate::types::PutV1CompaniesCompanyIdPaySchedulesScheduleRequest,
    ) -> Result<crate::types::PaySchedule> {
        let url = format!(
            "/v1/companies/{}/pay_schedules/{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            crate::progenitor_support::encode_path(&pay_schedule_id_or_uuid.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}