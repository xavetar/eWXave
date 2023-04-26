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

pub struct HeadTailString {
    head: String,
    tail: String,
}

impl HeadTailString {
    pub fn new(head: &'static str, tail: &'static str) -> HeadTailString {
        return HeadTailString {
            head: head.to_owned(),
            tail: tail.to_owned()
        }
    }
}

impl ToString for HeadTailString {
    fn to_string(&self) -> String {
        return self.head.to_owned() + &self.tail;
    }
}
