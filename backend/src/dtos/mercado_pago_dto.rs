use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MercadoPagoPreferenceRequest {
    pub items: Vec<MercadoPagoItem>,
    pub back_urls: BackUrls,
}

#[derive(Serialize)]
pub struct MercadoPagoItem {
    pub title: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Serialize)]
pub struct BackUrls {
    pub success: String,
    pub failure: String,
    pub pending: String,
}

#[derive(Deserialize)]
pub struct MercadoPagoPreferenceResponse {
    pub init_point: String,
    pub sandbox_init_point: String,
}
