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
use super::URLs::{Edge};

pub enum Profile {
    Me,
    Identification,
    DowngradeIdentification,
    Limits,
    Restrictions,
    History,
    Statistic,
    TransactionInfo,
    ReceiptFile,
    ReceiptEmail,
    ListBalances,
    CreateBalance,
    AvailableCurrencyBalances,
    SetDefaultBalance
}

impl Profile {
    fn url(&self) -> String {
        return match self {
            Profile::Me => EndpointUrl::new(
                Edge.url(),
                "person-profile/v1/profile/current"
            ).to_string(),
            Profile::Identification => EndpointUrl::new(
                Edge.url(),
                "identification/v1/persons/{}/identification"
            ).to_string(),
            Profile::DowngradeIdentification => EndpointUrl::new(
                Edge.url(),
                "/qw-ident-downgrade-api/v1/persons/wallet/identification-downgrade/operations"
            ).to_string(),
            Profile::Limits => EndpointUrl::new(
                Edge.url(),
                "qw-limits/v1/persons/{}/actual-limits"
            ).to_string(),
            Profile::Restrictions => EndpointUrl::new(
                Edge.url(),
                "person-profile/v1/persons/{}/status/restrictions"
            ).to_string(),
            Profile::History => EndpointUrl::new(
                Edge.url(),
                "payment-history/v2/persons/{}/payments"
            ).to_string(),
            Profile::Statistic => EndpointUrl::new(
                Edge.url(),
                "payment-history/v2/persons/{}/payments/total"
            ).to_string(),
            Profile::TransactionInfo => EndpointUrl::new(
                Edge.url(),
                "payment-history/v2/transactions/{}"
            ).to_string(),
            Profile::ReceiptFile => EndpointUrl::new(
                Edge.url(),
                "payment-history/v1/transactions/{}/cheque/file"
            ).to_string(),
            Profile::ReceiptEmail => EndpointUrl::new(
                Edge.url(),
                "payment-history/v1/transactions/{}/cheque/send"
            ).to_string(),
            Profile::ListBalances | Profile::CreateBalance => EndpointUrl::new(
                Edge.url(),
                "/funding-sources/v2/persons/{}/accounts"
            ).to_string(),
            Profile::AvailableCurrencyBalances => EndpointUrl::new(
                Edge.url(),
                "funding-sources/v2/persons/{}/accounts/offer"
            ).to_string(),
            Profile::SetDefaultBalance => EndpointUrl::new(
                Edge.url(),
                "funding-sources/v2/persons/{}/accounts/{}"
            ).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_url() {
        let url = super::Profile::Me.url();
        println!("{}", url);
    }
}