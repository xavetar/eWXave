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

pub enum WebHooks {
    Register,
    Remove,
    GetSecretKey,
    ChangeSecretKey,
    Active,
    Test
}

impl WebHooks {
    pub fn url(&self) -> String {
        return match self {
            WebHooks::Register => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks"
            ).to_string(),
            WebHooks::Remove => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks/{}"
            ).to_string(),
            WebHooks::GetSecretKey => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks/{}/key"
            ).to_string(),
            WebHooks::ChangeSecretKey => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks/{}/newkey"
            ).to_string(),
            WebHooks::Active => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks/active"
            ).to_string(),
            WebHooks::Test => EndpointUrl::new(
                Edge.url(),
                "payment-notifier/v1/hooks/test"
            ).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_url() {
        let url = super::WebHooks::Register.url();
        println!("{}", url);
    }
}
