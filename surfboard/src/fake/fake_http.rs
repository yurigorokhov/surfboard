use surfboard_lib::{data::HttpDataProvider, surf_report::SurfReportResponse};

#[derive(Default)]
pub struct FakeHttpClient;

// todo: support json data provider
impl HttpDataProvider<SurfReportResponse> for FakeHttpClient {
    async fn get_as_json<'a>(&'a self, _url: &'a str) -> Option<SurfReportResponse> {
        let fake_surf_report = include_bytes!("../../../surfboard_scraper/tests/data/surf_report.json");
        let (data, _remainder) = serde_json_core::from_slice::<SurfReportResponse>(fake_surf_report).unwrap();
        Some(data)
    }
}
