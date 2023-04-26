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
use super::URLs::{Oplata, API};

pub enum Kassa {
    Invoice,
    CheckInvoiceStatus,
    RejectInvoice,
    Refund,
    RefundStatus,
    InvoiceForm,
}

impl Kassa {
    pub fn url(&self) -> String {
        return match self {
            Kassa::Invoice | Kassa::CheckInvoiceStatus => EndpointUrl::new(
                API.url(),
                "partner/bill/v1/bills/{}"
            ).to_string(),
            Kassa::RejectInvoice => EndpointUrl::new(
                API.url(),
                "partner/bill/v1/bills/{}/reject"
            ).to_string(),
            Kassa::Refund | Kassa::RefundStatus => EndpointUrl::new(
                API.url(),
                "partner/bill/v1/bills/{}/refunds/{}"
            ).to_string(),
            Kassa::InvoiceForm => EndpointUrl::new(
                Oplata.url(),
                "create"
            ).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_url() {
        let url = super::Kassa::Invoice.url();
        println!("{}", url);
    }
}
