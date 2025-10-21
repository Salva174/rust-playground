use std::ffi::CStr;
use std::ptr;
use libc::{localtime_r, strftime, time, time_t, tm};
use crate::types::Topping;


//Zeitstempel über libc
fn now_local_timestamp() -> String {
    unsafe {
        let now_time: time_t = time(ptr::null_mut());
        let mut local_time: tm = std::mem::zeroed();

        if localtime_r(&now_time, &mut local_time).is_null() {
            return String::from("1970-01-01 00:00:00");     //Fallback
        }

        //"YYYY-MM-DD HH:MM:SS" -> max 19 + NUL
        let mut buf = [0u8; 32];
        let format = b"%Y-%m-%d %H:%M:%S\0";

        let written = strftime(
            buf.as_mut_ptr() as *mut _,
            buf.len(),
            format.as_ptr() as *const _,
            &local_time as *const tm,
        );

        if written == 0 {
            return String::from("1970-01-01 00:00:00");     //Fallback
        }

        // C-String -> Rust-String
        let cstr = CStr::from_ptr(buf.as_ptr() as *const i8);
        cstr.to_string_lossy().into_owned()

    }
}

fn format_eur_cents(cents: u32) -> String {
    let euros = cents / 100;
    let cents = cents % 100;
    format!("{euros},{cents:02}")
}

pub fn format_transaction_as_string(price_cents: u32, name: &str) -> String {
    let now = now_local_timestamp();
    let clean_name = name.replace(['\n', '\r'], " ");
    format!("{now};{};{}", format_eur_cents(price_cents), clean_name).to_string()
}

pub fn build_custom_name(available: &[Topping], qty: &[u32], include_qty: bool) -> String {
    let mut parts = Vec::new();
    for (i, &q) in qty.iter().enumerate() {
        if q == 0 { continue; }
        let name = &available[i].name;
        if include_qty && q > 1 {
            parts.push(format!("{name} x{q}"));
        } else {
            parts.push(name.clone());
        }
    }
    if parts.is_empty() {
        "Custom-Pizza".to_string()
    } else {
        format!("Custom-Pizza ({})", parts.join(", "))
    }
}

fn calc_custom_total_cents(base_price_eur: u32, available: &[Topping], qty: &[u32]) -> u32 {
    let toppings_sum_eur: u32 = qty.iter()
        .enumerate()
        .map(|(i, &q)| q * available[i].price)
        .sum();
    (base_price_eur + toppings_sum_eur) * 100
}

pub fn format_custom_pizza_as_transaction_string(
    base_price_eur: u32,
    available: &[Topping],
    qty: &[u32],        //Anzahl Toppings
    include_qty_in_name: bool
) -> String {
    let name = build_custom_name(available, qty, include_qty_in_name);
    let total_cents = calc_custom_total_cents(base_price_eur, available, qty);
    format_transaction_as_string(total_cents, &name)
}
