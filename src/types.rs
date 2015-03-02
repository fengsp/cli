// This module implements a number of types..
// Copyright (c) 2015 by Shipeng Feng.
// Licensed under the BSD License, see LICENSE for more details.


/// Command params type.
pub type Params = Vec<String>;


/// Command callback func type.
pub type CommandCallback = fn(Params);
