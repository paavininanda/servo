/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
/*
 * The origin of this IDL file is
 * https://drafts.fxtf.org/geometry-1/#Doge
 *
 * Copyright:
 * To the extent possible under law, the editors have waived all copyright and
 * related or neighboring rights to this work.
 */

typedef sequence<DOMString> DogeInit;

[Constructor(optional DogeInit init),
 Exposed=(Window,Worker)]
interface Doge {
  void append(DOMString word);
  [Throws] DOMString random();
  [Throws] void Remove(DOMString word);
  void printAll();
};
