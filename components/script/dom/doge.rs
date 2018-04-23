/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DomRefCell;
use dom::bindings::codegen::Bindings::DogeBinding::{DogeMethods, DogeInit, Wrap as DogeWrap};
use dom::bindings::error::{Error, Fallible};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::root::DomRoot;
use dom::bindings::str::DOMString;
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use servo_rand;
use servo_rand::Rng;

#[dom_struct]
pub struct Doge {
    reflector_: Reflector,
    such_list: DomRefCell<Vec<DOMString>>,
}

impl Doge {
    pub fn new_inherited() -> Doge {
        Doge {
            reflector_: Reflector::new(),
            such_list: DomRefCell::new(vec![]),
        }
    }

    pub fn new(global: &GlobalScope) -> DomRoot<Doge> {
        reflect_dom_object(Box::new(Doge::new_inherited()), global, DogeWrap)
    }

    pub fn Constructor(global: &GlobalScope, init: Option<DogeInit>) -> Fallible<DomRoot<Doge>> {
        let doge = Doge::new(global);
        if let Some(i) = init {
            for word in i {
                doge.Append(word);
            }
        }
        Ok(doge)
    }
}

impl DogeMethods for Doge {
    // https://drafts.fxtf.org/geometry-1/#doge1
    fn Append(&self, word: DOMString) {
        self.such_list.borrow_mut().push(word);
    }

    // https://drafts.fxtf.org/geometry-1/#doge2
    fn Random(&self) -> Fallible<DOMString> {
        let list = self.such_list.borrow();
        if list.len() == 0 {
            return Err(Error::Type("Such list is empty".to_string()));
        } else {
            let random_index = servo_rand::thread_rng().gen_range(0, list.len());
            return Ok(list[random_index].clone());
        }
    }

    // https://drafts.fxtf.org/geometry-1/#doge3
    fn Remove(&self, word: DOMString) -> Fallible<()> {
        let mut list = self.such_list.borrow_mut();
        let mut flag = 0;
        let mut index = 0;
        for x in 0..list.len() {
                if word == list[x] {
                flag = 1;
                index = x;
                break;
            }
        }
        if flag == 0 {
            return Err(Error::Type("Word not found".to_string()));
        } else {
            list.remove(index);
            return Ok(());
        }
    }

    // https://drafts.fxtf.org/geometry-1/#doge4
    fn PrintAll(&self) {
        let list = self.such_list.borrow();
        for x in 0..list.len() {
            println!("{}", list[x]);
        }
    }
}
