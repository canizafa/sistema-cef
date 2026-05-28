use crate::dtos::mercado_pago_dto::{
    BackUrls, MercadoPagoItem, MercadoPagoPreferenceRequest, MercadoPagoPreferenceResponse,
};
use reqwest::Client;
use std::env;

pub async fn crear_preferencia(
    titulo: String,
    monto: f64,
) -> Result<MercadoPagoPreferenceResponse, reqwest::Error> {
    let token = env::var("MERCADO_PAGO_ACCESS_TOKEN").expect("No existe MP token");

    let body = MercadoPagoPreferenceRequest {
        items: vec![MercadoPagoItem {
            title: titulo,
            quantity: 1,
            unit_price: monto,
        }],
        back_urls: BackUrls {
            success: "http://localhost:5173/pago/exito".to_string(),
            failure: "http://localhost:5173/pago/fallo".to_string(),
            pending: "http://localhost:5173/pago/pendiente".to_string(),
        },
        auto_return: "approved".to_string(),
    };

    let client = Client::new();
    let response = client
        .post("https://api.mercadopago.com/checkout/preferences")
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    let data = response.json::<MercadoPagoPreferenceResponse>().await?;
    Ok(data)
}
