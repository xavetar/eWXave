/*
 * Copyright (c) 2023 Stanislav Mikhailov (xavetar)
 *
 * Permission is hereby granted, free of charge, to any non-commercial company
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to use the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * Any commercial use of the Software requires a separate commercial license
 * from the copyright holder.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

use super::{EndpointUrl};
use super::URLs::{Base, Oplata, Edge, API};

pub enum Payments {
    Commission,
    AutofillForm,
    SendToQiwi,
    Convert,
    CurrencyRates,
    Payments,
    DetectCard,
    DetectMobile,
    SearchPhrase
}

impl Payments {
    pub fn url(&self) -> String {
        return match self {
            Payments::Commission => EndpointUrl::new(
                Edge.url(),
                "sinap/providers/{}/onlineCommission"
            ).to_string(),
            Payments::AutofillForm => EndpointUrl::new(
                Edge.url(),
                "payment/form/{}"
            ).to_string(),
            Payments::SendToQiwi => EndpointUrl::new(
                Edge.url(),
                "sinap/api/v2/terms/99/payments"
            ).to_string(),
            Payments::Convert => EndpointUrl::new(
                Edge.url(),
                "sinap/api/v2/terms/1099/payments"
            ).to_string(),
            Payments::CurrencyRates => EndpointUrl::new(
                Edge.url(),
                "sinap/crossRates"
            ).to_string(),
            Payments::Payments => EndpointUrl::new(
                Edge.url(),
                "sinap/api/v2/terms/{}/payments"
            ).to_string(),
            Payments::DetectCard => EndpointUrl::new(
                Base.url(),
                "card/detect.action"
            ).to_string(),
            Payments::DetectMobile => EndpointUrl::new(
                Base.url(),
                "mobile/detect.action"
            ).to_string(),
            Payments::SearchPhrase => EndpointUrl::new(
                Base.url(),
                "search/results/json.action?searchPhrase={}"
            ).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_url() {
        let url = super::Payments::Commission.url();
        println!("{}", url);
    }
}
