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

pub enum Master {
    BuyMasterPackage,
    CreateOrder,
    ConfirmationOrder,
    BuyCard,
    ListActiveCards,
    CardTransactions,
    BlockCard,
    UnblockCard,
    CardPaymentDetails,
    RenameCard
}

impl Master {
    pub fn url(&self) -> String {
        return match self {
            Master::BuyMasterPackage => EndpointUrl::new(
                Edge.url(),
                "sinap/api/v2/terms/28004/payments"
            ).to_string(),
            Master::CreateOrder => EndpointUrl::new(
                Edge.url(),
                "/cards/v2/persons/{}/orders"
            ).to_string(),
            Master::ConfirmationOrder => EndpointUrl::new(
                Edge.url(),
                "cards/v2/persons/{}/orders/{}/submit"
            ).to_string(),
            Master::BuyCard => EndpointUrl::new(
                Edge.url(),
                "sinap/api/v2/terms/32064/payments"
            ).to_string(),
            Master::ListActiveCards => EndpointUrl::new(
                Edge.url(),
                "cards/v1/cards/?vas-alias={}"
            ).to_string(),
            Master::CardTransactions => EndpointUrl::new(
                Edge.url(),
                "payment-history/v1/persons/{}/cards/{}/statement?from={}&till={}"
            ).to_string(),
            Master::BlockCard => EndpointUrl::new(
                Edge.url(),
                "cards/v2/persons/{}/cards/{}/block"
            ).to_string(),
            Master::UnblockCard => EndpointUrl::new(
                Edge.url(),
                "cards/v2/persons/{}/cards/{}/unblock"
            ).to_string(),
            Master::CardPaymentDetails => EndpointUrl::new(
                Edge.url(),
                "cards/v1/cards/{}/details"
            ).to_string(),
            Master::RenameCard => EndpointUrl::new(
                Edge.url(),
                "cards/v1/cards/{}/alias"
            ).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_url() {
        let url = super::Master::BuyMasterPackage.url();
        println!("{}", url);
    }
}
